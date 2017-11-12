use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{Session, SessionHandler};
use std::error::Error;
use uuid::Uuid;
use serde_json;
use postgres::rows;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    given_name: String,
    family_name: String,
}

impl Name {
    pub fn from_row(row: &rows::Row) -> Name {
        Name {
            given_name: row.get("given_name"),
            family_name: row.get("family_name")
        }
    }
}

impl SessionHandler for Handler {
    fn handle_session(&self, _: &mut Session, req: &mut Request) -> IronResult<Response> {
        let user_id: Uuid = match util::get_url_param(req, "user_id") {
            None => return Ok(Response::with((status::BadRequest, "no user_id"))),
            Some(ref user_id) => match Uuid::parse_str(user_id.as_ref()) {
                Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
                Ok(user_id) => user_id,
            }
        };
        let name = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query(
                "SELECT * FROM get_user_name($1)",
                &[&user_id]) {
                Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    Name::from_row(&rows.get(0))
                } else {
                    return Ok(Response::with((status::NotFound, "user not found")));
                }
            }
        };
        match serde_json::to_string(&name) {
            Err(err) => Ok(Response::with((status::InternalServerError, err.description() ))),
            Ok(name) => Ok(Response::with((status::Ok, name)))
        }
    }
}