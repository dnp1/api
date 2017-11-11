use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{Session, SessionHandler};
use std::error::Error;
use uuid::Uuid;
use postgres::rows;
//use serde;
use serde_json;

#[derive(Clone, Serialize, Deserialize)]
struct Resp {
    id: Uuid,
    title: String,
    publication_datetime: i64,
    edition_datetime: i64,
}

impl Resp {
    fn from_row(row: &rows::Row) -> Resp {
        return Resp {
            id: row.get("id"),
            title: row.get("title"),
            publication_datetime: row.get("publication_datetime"),
            edition_datetime: row.get("edition_datetime")
        };
    }
}

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}


const FETCH_LENGTH: i32 = 10;

impl SessionHandler for Handler {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let after_uuid: Option<Uuid> = None;
        let resp : Vec<Resp> = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article_list($1, $2)",
                                                     &[&FETCH_LENGTH, &after_uuid]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => (&rows).iter().map(|row| Resp::from_row(&row)).collect()
            }
        };

        match serde_json::to_string(&resp) {
            Err(err) => Ok(Response::with((status::InternalServerError,err.description()))),
            Ok(json) => Ok(Response::with((status::Ok,json)))
        }
    }
}