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
use util::FromRouteParams;
use std::str::FromStr;
use router::Router;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

#[derive(FromRouteParams)]
struct RouteParams {
    article_id: Uuid,
    comment_id: Uuid
}

impl SessionHandler for Handler {
    fn handle(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let params = req.extensions.get::<Router>().unwrap();
        let a = match RouteParams::from_route_params(params) {
            Err(_) => return Ok(Response::with((status::BadRequest, "no article_id"))),
            Ok(data) => data,
        };
        let comment = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article_comment($1, $2)",
                                                     &[&a.article_id, &a.comment_id]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    Comment::from_row(&rows.get(0))
                } else {
                    return Ok(Response::with((status::NotFound, "not found")));
                }
            }
        };
        Ok(Response::with((status::Ok, json(comment))))
    }
}