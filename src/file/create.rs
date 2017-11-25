use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use iron::prelude::Response;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::prelude::*;
use iron::status;
use params::Params;
use params::Value;
use util::{Session, SessionHandler, Storage};
use uuid::Uuid;
use std::error::Error;
use std::io::BufReader;
use util::json;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct File {
    id: Uuid
}

pub struct Handler<T : Storage> {
    pub db: Arc<Pool<PostgresConnectionManager>>,
    pub storage: Arc<T>,
}

impl <T>SessionHandler for Handler<T> where T : Storage {
    fn authenticated(&self) -> bool {
        true
    }
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let file = match req.get_ref::<Params>() {
            Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
            Ok(params) => match params.find(&["file"]) {
                Some(file) => match *file {
                    Value::File(ref file) => file,
                    _ => return Ok(Response::with((status::BadRequest, "")))
                }
                None => return Ok(Response::with((status::BadRequest, "")))
            }
        };
        let mut opened_file = match file.open() {
            Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
            Ok(opened_file) => BufReader::with_capacity(32768, opened_file),
        };

        let file_id: Uuid = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(db) => {
                match db.query(
                    "SELECT create_file($1, $2, $3, $4) AS id",
                    &[
                        &(file.filename),
                        &(file.size as i64),
                        &file.content_type.to_string(),
                        &session.user_id,
                    ]
                ) {
                    Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                    Ok(rows) => {
                        if rows.len() > 0 {
                            rows.get(0).get("id")
                        } else {
                            return Ok(Response::with((status::InternalServerError, "no file created")))
                        }
                    }
                }
            }
        };

        if let Err(e) =  self.storage.save(&file_id.simple().to_string(), &mut opened_file) {
            return Ok(Response::with((status::InternalServerError, e.description())))
        };

        Ok(Response::with((status::Ok, json(&File{id:file_id}))))
    }
}