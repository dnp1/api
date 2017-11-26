use iron::prelude::*;
use iron::status;
use iron;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{SessionManager, Session, json};
use std::error::Error;
use user::common::ExposedSession;
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
            let mut response = Response::with((status::Ok, json(&ExposedSession{user_id: None})));
            set_cookie(&mut response, &session);
            Ok(response)
        } else {
            Ok(Response::with((status::ServiceUnavailable, "")))
        }
    }
}