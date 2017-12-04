use iron::status;
use iron::Response;
use iron::IronResult;

use uuid::Uuid;
use iron_simple::SimpleHandler;

use article::common::Comment;
use util::json;
use super::{Session, Services};

use std::error::Error;


const FETCH_LENGTH: i32 = 10;

#[derive(RequestRouteParams)]
pub struct RouteParams {
    article_id: Uuid,
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, Session);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, session) = req;
        let after_uuid: Option<Uuid> = None; //TODO:get_query_param

        let resp: Vec<Comment> = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article_comment_list($1, $2, $3)",
                                                     &[&route_params.article_id, &after_uuid, &FETCH_LENGTH]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => (&rows).iter().map(|row| Comment::from_row(&row)).collect()
            }
        };
        Ok(Response::with((status::Ok, json(resp))))
    }
}