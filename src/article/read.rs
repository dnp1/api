use iron::status;
use iron::Response;
use iron::IronResult;

use uuid::Uuid;
use iron_simple::SimpleHandler;

use article::common::Article;
use util::json;
use super::{Session, Services};
use std::error::Error;

#[derive(RequestRouteParams)]
pub struct RouteParams {
    article_id: Uuid
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, Session,);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, _) = req;

        let resp = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article($1)", &[&route_params.article_id]) {
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
