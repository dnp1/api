use iron::status;
use iron::Response;
use iron::IronResult;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Session, SimpleHandler, Empty, SimpleRequest, FromRouteParams, json};
use std::error::Error;
use uuid::Uuid;
use article::common::Comment;
use std::str::FromStr;
use router::Router;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

#[derive(FromRouteParams)]
pub struct RouteParams {
    article_id: Uuid,
    comment_id: Uuid
}


impl SimpleHandler<RouteParams, Empty, Empty, Empty> for Handler {
    fn handle(&self, req: &SimpleRequest<RouteParams, Empty, Empty, Empty>, session: &mut Session) -> IronResult<Response> {
        let comment = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article_comment($1, $2)",
                                                     &[&req.route_params.article_id, &req.route_params.comment_id]) {
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