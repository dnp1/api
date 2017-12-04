use iron::status;
use iron::Response;
use iron::IronResult;

use uuid::Uuid;
use iron_simple::SimpleHandler;

use util::json;
use super::{AuthenticatedSession, Services};


use params::Params;
use params::Value;
use std::io::BufReader;
use util::storage::Storage;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct File {
    id: Uuid
}

pub struct Handler;

impl SimpleHandler for Handler {
//    fn handdasdle(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
//        let file = match req.get_ref::<Params>() {
//            Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
//            Ok(params) => match params.find(&["file"]) {
//                Some(file) => match *file {
//                    Value::File(ref file) => file,
//                    _ => return Ok(Response::with((status::BadRequest, "")))
//                }
//                None => return Ok(Response::with((status::BadRequest, "")))
//            }
//        };
//        let mut opened_file = match file.open() {
//            Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
//            Ok(opened_file) => BufReader::with_capacity(32768, opened_file),
//        };
//
//        let file_id: Uuid = match self.db.get() {
//            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
//            Ok(db) => {
//                match db.query(
//                    "SELECT create_file($1, $2, $3, $4) AS id",
//                    &[
//                        &(file.filename),
//                        &(file.size as i64),
//                        &file.content_type.to_string(),
//                        &session.user_id,
//                    ]
//                ) {
//                    Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
//                    Ok(rows) => {
//                        if rows.len() > 0 {
//                            rows.get(0).get("id")
//                        } else {
//                            return Ok(Response::with((status::InternalServerError, "no file created")))
//                        }
//                    }
//                }
//            }
//        };
//
//        if let Err(e) =  self.storage.save(&file_id.simple().to_string(), &mut opened_file) {
//            return Ok(Response::with((status::InternalServerError, e.description())))
//        };
//
//        Ok(Response::with((status::Ok, json(&File{id:file_id}))))
//    }
    type Services = Services;
    type Request = (AuthenticatedSession,);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        unimplemented!()
    }
}