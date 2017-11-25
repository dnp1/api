use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Session, SessionHandler};
use std::error::Error;
use uuid::Uuid;
use article::common::Article;
use util::Json;


pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}


const FETCH_LENGTH: i32 = 10;

impl SessionHandler for Handler {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let after_uuid: Option<Uuid> = None; //TODO:get_query_param
        let resp : Vec<Article> = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article_list($1, $2)",
                                                     &[&FETCH_LENGTH, &after_uuid]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => (&rows).iter().map(|row| Article::from_row(&row)).collect()
            }
        };
        Ok(Response::with((status::Ok,Json(resp))))
    }
}