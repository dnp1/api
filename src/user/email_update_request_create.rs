use iron::status;
use iron::Response;
use iron::IronResult;

use std::error::Error;
use uuid::Uuid;
use super::{AuthenticatedSession, Services};
use iron_simple::SimpleHandler;


#[derive(RequestRouteParams)]
pub struct RouteParams {
    user_id: Uuid
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, AuthenticatedSession);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        match services.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, "sdasdsad")))
        }
    }
}