use iron::prelude::*;
use iron::status;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Session, SimpleHandler, Empty, SimpleRequest, FromRouteParams};
use std::error::Error;
use uuid::Uuid;
use article::common::Comment;
use util::json;
use std::str::FromStr;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

const FETCH_LENGTH: i32 = 10;

#[derive(FromRouteParams)]
pub struct RouteParams {
    article_id: Uuid,
}

impl SimpleHandler<RouteParams, Empty, Empty, Empty> for Handler {
    fn handle(&self, req: &SimpleRequest<RouteParams, Empty, Empty, Empty>, session: &mut Session) -> IronResult<Response> {
        let after_uuid: Option<Uuid> = None; //TODO:get_query_param

        let resp: Vec<Comment> = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article_comment_list($1, $2, $3)",
                                                     &[&req.route_params.article_id, &after_uuid, &FETCH_LENGTH]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => (&rows).iter().map(|row| Comment::from_row(&row)).collect()
            }
        };
        Ok(Response::with((status::Ok, json(resp))))
    }
}