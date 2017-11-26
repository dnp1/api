use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{Session, SessionHandler};
use std::error::Error;
use uuid::Uuid;
use article::common::Content;
use util::json;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for Handler {
    fn handle(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let article_id: Uuid = match util::get_url_param(req, "article_id") {
            None => return Ok(Response::with((status::BadRequest, "no article_id"))),
            Some(ref user_id) => match Uuid::parse_str(user_id.as_ref()) {
                Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
                Ok(user_id) => user_id,
            }
        };
        let comment_id: Uuid = match util::get_url_param(req, "comment_id") {
            None => return Ok(Response::with((status::BadRequest, "no comment_id"))),
            Some(ref user_id) => match Uuid::parse_str(user_id.as_ref()) {
                Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
                Ok(user_id) => user_id,
            }
        };

        let comment_content = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT get_article_comment_content($1, $2) as content",
                                                     &[&article_id, &comment_id]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    Content::from_row(&rows.get(0))
                } else {
                    return Ok(Response::with((status::NotFound, "not found")))
                }
            }
        };
        Ok(Response::with((status::Ok, json(comment_content))))
    }
}