use iron::prelude::*;
use iron::status;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Session, SimpleHandler, SimpleRequest, FromRouteParams, Empty};
use std::error::Error;
use uuid::Uuid;
use article::common::Article;
use util::json;
use std::str::FromStr;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

#[derive(FromRouteParams)]
pub struct RouteParams {
    article_id: Uuid
}

impl SimpleHandler<RouteParams, Empty, Empty, Empty> for Handler {
    fn handle(&self,  req: & SimpleRequest<RouteParams, Empty, Empty, Empty>, session: &mut Session) -> IronResult<Response> {
        let resp = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article($1)", &[&req.route_params.article_id]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    Article::from_row(&rows.get(0))
                } else {
                    return Ok(Response::with((status::NotFound, "article was not found")))
                }
            }
        };
        Ok(Response::with((status::Ok, json(resp))))
    }
}
