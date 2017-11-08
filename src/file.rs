use iron::prelude::Response;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::status;
use params::Params;
use params::Value;

//use iron::IronError;
use iron::prelude::*;
use iron::Handler;
use std::sync::Arc;
use std::error::Error;

use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;


use std::io::prelude::*;
//use std::io::Error;
use std::fs::File;
use uuid::Uuid;

pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, r: &'s mut Router) {
    let db = Arc::new(db);
    r.post("/file", FileCreate { db: db.clone() }, "file_create");
    r.get("/file", FileRead { db: db.clone() }, "file_read");
    r.delete("/file", FileDelete { db: db.clone() }, "file_delete");
}

struct FileCreate { db: Arc<Pool<PostgresConnectionManager>> }

impl Handler for FileCreate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
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
        let opened_file =  match file.open() {
            Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
            Ok(opened_file) => opened_file,
        };

        let file_id: Uuid = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(db) => {
                match db.query(
                    "SELECT create_file($1, $2, $3, $4) AS id",
                    &[
                        &file.filename,
                        &file.content_type.to_string(),
                        &(file.size as i64)
                    ]
                ) {
                    Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                    Ok(rows) => {
                        let row = &rows.get(0);
                        row.get("id")
                    }
                }
            }
        };
        Ok(Response::with((status::ServiceUnavailable, "")))
    }
}


struct FileRead { db: Arc<Pool<PostgresConnectionManager>> }

impl Handler for FileRead {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, "")))
        }
    }
}

struct FileDelete { db: Arc<Pool<PostgresConnectionManager>> }

impl Handler for FileDelete {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, "")))
        }
    }
}


