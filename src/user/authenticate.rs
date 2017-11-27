use iron::prelude::*;
use iron::status;
use bodyparser;
use std::sync::Arc;
use std::error::Error;
use uuid::Uuid;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Session, SimpleHandler, Empty, SimpleRequest, FromBodyParser, json};
use user::common::ExposedSession;

#[derive(Clone, Serialize, Deserialize, FromBodyParser)]
pub struct RequestBody {
    email: String,
    password: String
}

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

impl SimpleHandler<Empty, Empty, RequestBody, Empty> for Handler {
    fn handle(&self, req: &SimpleRequest<Empty, Empty, RequestBody, Empty>, session: &mut Session) -> IronResult<Response> {
        let user_id: Uuid = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query(
                "SELECT authenticate($1, $2, $3, '192.168.43.37') as ok",
                &[&session.id, &req.body.email, &req.body.password]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => {
                    if rows.len() > 0 {
                        rows.get(0).get("ok")
                    } else {
                        return Ok(Response::with((status::Unauthorized, "")))
                    }
                }
            },
        };
        session.user_id = Some(user_id);
        Ok(Response::with((status::Ok, json(ExposedSession { user_id: Some(user_id) }))))

    }
}