use iron::prelude::Response;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::status;
use params::Params;
use params::Value;

//use iron::IronError;
use iron::prelude::*;
use std::sync::Arc;
use std::error::Error;

use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use std::io::BufReader;

//use std::io::Error;
use serde_json;
use uuid::Uuid;

use util::{SessionManager, Session, SessionHandler, SessionHandlerBox, Storage};

pub fn register_handlers<'s, T>(db: Pool<PostgresConnectionManager>, r: &'s mut Router, sm: Arc<SessionManager>, storage: T) where T : Storage {
    let db = Arc::new(db);
    let file_read = FileRead { db: db.clone() };
    let file_create = FileCreate { db: db.clone(), storage };
    let file_delete = FileDelete { db: db.clone() };
    r.post("/file", SessionHandlerBox { handler: file_create, sm: sm.clone() }, "file_create");
    r.get("/file", SessionHandlerBox { handler: file_read, sm: sm.clone() }, "file_read");
    r.delete("/file", SessionHandlerBox { handler: file_delete, sm: sm.clone() }, "file_delete");
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct File {
    id: Uuid
}

struct FileCreate<T : Storage> {
    db: Arc<Pool<PostgresConnectionManager>>,
    storage: T
}

impl <T>SessionHandler for FileCreate<T> where T : Storage {
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
                        &file.filename,
                        &file.content_type.to_string(),
                        &(file.size as i64),
                        &session.user_id,
                    ]
                ) {
                    Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                    Ok(rows) => {
                        let row = &rows.get(0);
                        row.get("id")
                    }
                }
            }
        };

        if let Err(e) =  self.storage.save(&file_id.simple().to_string(), &mut opened_file) {
            return Ok(Response::with((status::InternalServerError, e.description())))
        };

        match serde_json::to_string(&File{id:file_id}) {
            Err(err) => Ok(Response::with((status::InternalServerError, err.description()))),
            Ok(email) => Ok(Response::with((status::Ok, email)))
        }
    }
}


struct FileRead {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for FileRead {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, "")))
        }
    }
}

struct FileDelete {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for FileDelete {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "")))
    }
}

