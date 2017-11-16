use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use iron::prelude::Response;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::status;
use util::{Session, SessionHandler, Storage};
use std::error::Error;
use postgres::rows;
use uuid::Uuid;
use iron::mime;
use util;
use iron::response::BodyReader;
use iron::headers::ContentDisposition;
use iron::headers::ContentLength;
use hyper::header::{Headers, DispositionType, DispositionParam, Charset};


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

pub struct Handler<T : Storage> {
    pub db: Arc<Pool<PostgresConnectionManager>>,
    pub storage: Arc<T>,
}

impl <T>SessionHandler for Handler<T> where T: Storage {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let file_id: Uuid = match util::get_url_param(req, "file_id") {
            None => return Ok(Response::with((status::BadRequest, "no file_id"))),
            Some(ref user_id) => match Uuid::parse_str(user_id.as_ref()) {
                Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
                Ok(user_id) => user_id,
            }
        };

        let file = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query("SELECT * FROM get_file($1)", &[&file_id]) {
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
            match self.storage.retrieve(&file_id.simple().to_string()) {
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
