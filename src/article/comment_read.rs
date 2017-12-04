use iron::status;
use iron::Response;
use iron::IronResult;

use uuid::Uuid;
use iron_simple::SimpleHandler;

use article::common::Comment;
use util::json;
use super::{Session, Services};
use std::error::Error;


#[derive(RequestRouteParams)]
pub struct RouteParams {
    article_id: Uuid,
    comment_id: Uuid
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, Session);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, session) = req;

        let comment = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article_comment($1, $2)",
                                                     &[&route_params.article_id, &route_params.comment_id]) {
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