use iron::status;
use iron::Response;
use iron::IronResult;

use uuid::Uuid;
use iron_simple::SimpleHandler;

use util::json;
use super::{Session, Services};
use postgres::rows;
use iron::mime;
use iron::response::BodyReader;
use iron::headers::ContentDisposition;
use iron::headers::ContentLength;
use iron::headers::{DispositionType, DispositionParam, Charset};
use std::error::Error;
use util::storage::Storage;

struct File {
    size: i64,
    filename: String,
    mime: String,
}

impl File {
    fn from_row(row: rows::Row) -> File {
        File{
            size: row.get("size"),
            filename: row.get("filename"),
            mime: row.get("mime"),
        }
    }
}



#[derive(RequestRouteParams)]
pub struct RouteParams {
    file_id: Uuid
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, Session);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, _) = req;

        let file = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query("SELECT * FROM get_file($1)", &[&route_params.file_id]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    File::from_row(rows.get(0))
                } else {
                    return Ok(Response::with((status::NotFound, "file was not found")))
                }
            }
        };

        let content_type = match file.mime.parse::<mime::Mime>() {
            Err(_) => return Ok(Response::with((status::InternalServerError, ))),
            Ok(content_type) => content_type
        };

        let bufread =
            match services.storage.retrieve(&route_params.file_id.simple().to_string()) {
                Err(err) => return Ok(Response::with((status::InternalServerError, ))),
                Ok(bf) => bf,
            };

        let mut resp = Response::with((content_type, status::Ok, BodyReader(bufread)));
        resp.headers.set(ContentLength(file.size as u64));
        resp.headers.set(ContentDisposition{
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(
                Charset::Iso_8859_1, // The character set for the bytes of the filename
                None, // The optional la0nguage tag (see `language-tag` crate)
                file.filename.as_bytes().to_vec()// the actual bytes of the filename
            )]});

        Ok(resp)
    }
}
