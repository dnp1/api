use iron::prelude::*;
use iron::status;
use iron;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{SessionManager, Session};
use std::error::Error;
use user::common::ExposedSession;
use serde_json;
use iron::headers::{SetCookie};
use util::TOKEN_NAME;
use util::{set_cookie};

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
    pub sm: Arc<SessionManager>,
}


impl iron::Handler for Handler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let db = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => connection
        };
        let session_id: i64 = match db.query("SELECT create_session() as id", &[]) {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(rows) => {
                let row = &rows.get(0);
                row.get("id")
            }
        };

        if let Ok(session) = self.sm.create_session_payload(&mut Session::new(session_id)) {
            let mut response = match serde_json::to_string(&ExposedSession{user_id: None}) {
                Err(err) => Response::with((status::InternalServerError, err.description())),
                Ok(json) => Response::with((status::Ok, json)),
            };
            set_cookie(&mut response, &session);
            Ok(response)
        } else {
            Ok(Response::with((status::ServiceUnavailable, "")))
        }
    }
}