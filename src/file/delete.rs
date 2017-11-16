use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use iron::prelude::Response;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::status;
use util::{Session, SessionHandler, Storage};
use std::error::Error;
use uuid::Uuid;
use util;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for Handler {
    fn authenticated(&self) -> bool {
        true
    }
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let file_id: Uuid = match util::get_url_param(req, "file_id") {
            None => return Ok(Response::with((status::BadRequest, "no file_id"))),
            Some(ref user_id) => match Uuid::parse_str(user_id.as_ref()) {
                Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
                Ok(user_id) => user_id,
            }
        };

        let ok: Option<bool> = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query("SELECT deactivate_file($1, $2)",
                                         &[&file_id, &session.user_id]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    rows.get(0).get(0)
                } else {
                    return Ok(Response::with((status::NotFound, "file was not found")))
                }
            }
        };
        match ok {
            None => Ok(Response::with((status::NotFound, "file was not found"))),
            Some(false) =>  Ok(Response::with((status::Forbidden, "You don't have authorization for delete this file"))),
            Some(true) =>  Ok(Response::with((status::Ok, "")))
        }
    }
}