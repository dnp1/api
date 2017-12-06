use iron::status;
use iron::Response;
use iron::IronResult;

use iron::Request;
use iron::Plugin;

use uuid::Uuid;
use iron_simple::SimpleHandler;
use iron_simple::SimpleResult;
use iron_simple::FromIronRequest;
use iron_simple::{SimpleError, ClientError, ServerError};

use util::json;
use super::{AuthenticatedSession, Services};


use params::Params;
use params::Value;
use std::io::BufReader;
use util::storage::Storage;
use std::error::Error;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    id: Uuid
}

impl FromIronRequest<Services> for File {
    fn from_request<'a>(req: &mut Request, services: &Services) -> SimpleResult<Self> {
        let session = match AuthenticatedSession::from_request(req, services) {
            Err(err) => return Err(err),
            Ok(session) => session,
        };

        let file = match req.get_ref::<Params>() {
            Err(err) => return Err(SimpleError::Server(ServerError::PluginNotFound(err.description().to_owned()))),
            Ok(params) => match params.find(&["file"]) {
                Some(file) => match *file {
                    Value::File(ref file) => file,
                    _ => return Err(SimpleError::Client(ClientError::InvalidBody("Invalid file sent.".to_owned())))
                }
                None => return Err(SimpleError::Client(ClientError::InvalidBody("Empty file was given. A valid file is mandatory.".to_owned())))
            }
        };
        let mut opened_file = match file.open() {
            Err(err) => return Err(SimpleError::Server(ServerError::Other(err.description().to_owned()))),
            Ok(opened_file) => BufReader::with_capacity(32768, opened_file),
        };

        let file_id: Uuid = match services.db.get() {
            Err(err) => return Err(SimpleError::Server(ServerError::ServiceUnavailable(err.description().to_owned()))),
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
                    Err(err) => return Err(SimpleError::Server(ServerError::Other(err.description().to_owned()))),
                    Ok(rows) => {
                        if rows.len() > 0 {
                            rows.get(0).get("id")
                        } else {
                            return Err(SimpleError::Server(ServerError::Other("no file created".to_owned())))
                        }
                    }
                }
            }
        };

        if let Err(err) =  services.storage.save(&file_id.simple().to_string(), &mut opened_file) {
            return Err(SimpleError::Server(ServerError::Other(err.description().to_owned())))
        };
        Ok(File {id: file_id})
    }
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (File,);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (file,) = req;
        Ok(Response::with((status::Ok, json(&file))))
    }
}