use iron::prelude::*;
use iron::status;
use bodyparser;
use std::sync::Arc;
use std::error::Error;
use uuid::Uuid;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Session, SessionHandler};

#[derive(Clone, Serialize, Deserialize)]
struct AuthenticateBody {
    email: String,
    password: String
}

pub struct Authenticate {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for Authenticate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let body = req.get::<bodyparser::Struct<AuthenticateBody>>();
        let body = match body {
            Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
            Ok(None) => return Ok(Response::with((status::BadRequest, "empty body"))),
            Ok(Some(struct_body)) => struct_body,
        };

        let user_id: Option<Uuid> = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query(
                "SELECT authenticate($1, $2, $3, '192.168.43.37') as ok",
                &[&session.id, &body.email, &body.password]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => {
                    if rows.len() > 0 {
                        rows.get(0).get("ok")
                    } else {
                        None
                    }
                }
            },
        };
        if let Some(id) = user_id {
            session.user_id = user_id;
            Ok(Response::with((status::Ok, id.simple().to_string())))
        } else {
            Ok(Response::with((status::Unauthorized, "")))
        }
    }
}