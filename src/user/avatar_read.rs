use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{Session, SessionHandler};
use std::error::Error;
use uuid::Uuid;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for Handler {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let user_id: Uuid = match util::get_url_param(req, "user_id") {
            None => return Ok(Response::with((status::BadRequest, "no user_id"))),
            Some(ref user_id) => match Uuid::parse_str(user_id.as_ref()) {
                Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
                Ok(user_id) => user_id,
            }
        };
        let avatar_id: Option<Uuid> = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query(
                "SELECT get_user_avatar($1) as file_id",
                &[&user_id]) {
                Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    rows.get(0).get("file_id")
                } else {
                    return Ok(Response::with((status::NotFound, "avatar not found for user_id")))
                }
            }
        };
        match avatar_id {
            None => Ok(Response::with((status::NotFound, ))),
            Some(avatar_id) => Ok(Response::with((status::Ok, avatar_id.simple().to_string())))
        }
    }
}
