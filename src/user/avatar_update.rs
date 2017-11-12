use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use std::error::Error;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{Session, SessionHandler};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
struct RequestBody {
    email: String,
    password: String
}


pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for Handler {
    fn authenticated(&self) -> bool { true }
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let user_id: Uuid = match util::get_url_param(req, "user_id") {
            None => return Ok(Response::with((status::BadRequest, "no user_id"))),
            Some(ref user_id) => match Uuid::parse_str(user_id.as_ref()) {
                Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
                Ok(user_id) => user_id,
            }
        };

        let _ = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => {
                match connection.query(
                    "SELECT TODO:($1) as user_id",
                    &[&user_id]) {
                    Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                    Ok(rows) => if rows.len() > 0 {
                        rows.get(0).get("id")
                    } else {
                        1
                    }
                }
            }
        };
        Ok(Response::with((status::NotImplemented, "")))
    }
}