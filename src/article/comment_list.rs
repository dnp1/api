use iron::prelude::*;
use iron::status;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{Session, SessionHandler};
use std::error::Error;
use uuid::Uuid;
use article::common::Comment;
use util::json;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

const FETCH_LENGTH: i32 = 10;


impl SessionHandler for Handler {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let article_id: Uuid = match util::get_url_param(req, "article_id") {
            None => return Ok(Response::with((status::BadRequest, "no article_id"))),
            Some(ref user_id) => match Uuid::parse_str(user_id.as_ref()) {
                Err(err) => return Ok(Response::with((status::BadRequest, err.description()))),
                Ok(user_id) => user_id,
            }
        };
        let after_uuid: Option<Uuid> = None; //TODO:get_query_param

        let resp: Vec<Comment> = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article_comment_list($1, $2, $3)",
                                                     &[&article_id, &after_uuid, &FETCH_LENGTH]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => (&rows).iter().map(|row| Comment::from_row(&row)).collect()
            }
        };
        Ok(Response::with((status::Ok, json(resp))))
    }
}