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


pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, r: &'s mut Router) {
    let db = Arc::new(db);
    r.post("/file", FileCreate { db: db.clone() }, "file_create");
    r.get("/file", FileRead { db: db.clone() }, "file_read");
    r.delete("/file", FileDelete { db: db.clone() }, "file_delete");
}

struct FileCreate { db: Arc<Pool<PostgresConnectionManager>> }

impl Handler for FileCreate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => {
                let params = req.get_ref::<Params>();
                match params {
                    Err(err) => Ok(Response::with((status::BadRequest, err.description()))),
                    Ok(params) => {
                        let file = params.find(&["file"]);
                        if let Some(file) = file {
                            if let Value::File(ref file) = *file {
                                return match file.open() {
                                    Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
                                    Ok(file) => {
                                        Ok(Response::with((status::Ok , "csdsa")))
                                    }
                                }
                            }
                        }
                        Ok(Response::with((status::BadRequest , "")))

                    }
                }
            }
        }
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


