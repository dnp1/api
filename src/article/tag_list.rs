use iron::status;
use iron::Response;
use iron::IronResult;

//use uuid::Uuid;
use iron_simple::SimpleHandler;

use util::json;
use super::{Session, Services};
use std::error::Error;

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (Session,);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let ref article_id = "dsada";

        match services.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, json(article_id))))
        }
    }
}