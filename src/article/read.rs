use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{Session, SessionHandler};
use std::error::Error;
use uuid::Uuid;
use article::common::Article;
use serde_json;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for Handler {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let article_id: Uuid = match util::get_url_param(req, "article_id") {
            None => return Ok(Response::with((status::BadRequest, "no article_id"))),
            Some(ref user_id) => match Uuid::parse_str(user_id.as_ref()) {
                Err(err) => {
                    let mut msg = err.description().to_owned();
                    msg.push_str(*user_id);
                    return Ok(Response::with((status::BadRequest, msg)));
                }
                Ok(user_id) => user_id,
            }
        };
        let resp = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article($1)",
                                                     &[&article_id]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    Article::from_row(&rows.get(0))
                } else {
                    return Ok(Response::with((status::NotFound, "article was not found")))
                }
            }
        };

        match serde_json::to_string(&resp) {
            Err(err) => Ok(Response::with((status::InternalServerError,err.description()))),
            Ok(json) => Ok(Response::with((status::Ok,json)))
        }
    }
}
