use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{Session, SessionHandler};
use std::error::Error;


pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for Handler {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param_default(req, "article_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}