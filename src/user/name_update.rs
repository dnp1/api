use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{Session, SessionHandler};
use std::error::Error;
use bodyparser;
use uuid::Uuid;
use serde_json;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestBody {
    given_name: String,
    family_name: String,
    password: String
}

#[derive(Clone, Serialize, Deserialize)]
struct ResponseBody {
    success: bool
}

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>
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

        let body = match req.get::<bodyparser::Struct<RequestBody>>() {
            Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
            Ok(None) => return Ok(Response::with((status::BadRequest, "empty body"))),
            Ok(Some(struct_body)) => struct_body,
        };

        if session.user_id != Some(user_id) {
            return Ok(Response::with((status::Forbidden, "you can only update only your self")))
        }


        let password_match : bool = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => {
                match connection.query(
                    "SELECT set_user_name($1, $2, $3, $4) as password_match",
                    &[&user_id, &body.given_name, &body.family_name, &body.password]) {
                    Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                    Ok(rows) => if rows.len() > 0 {
                        rows.get(0).get("password_match")
                    } else {
                        return Ok(Response::with((status::NotFound, "user not found")))
                    }
                }
            }
        };

        let status_code = if password_match {
            status::Ok
        } else {
            status::Unauthorized
        };
        let resp = match serde_json::to_string(&ResponseBody {success: password_match}) {
            Err(err) =>  return Ok(Response::with((status::InternalServerError, err.description()))),
            Ok(json) => json,
        };

        Ok(Response::with((status_code, resp)))
    }
}