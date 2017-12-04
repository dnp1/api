use iron::status;
use iron::Response;
use iron::IronResult;

use std::error::Error;
use iron_simple::SimpleHandler;
use uuid::Uuid;
use super::{AuthenticatedSession, Services};


pub struct Handler;

#[derive(RequestRouteParams)]
pub struct RouteParams {
    user_id: Uuid
}

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, AuthenticatedSession);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, _) = req;

        match services.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, route_params.user_id.simple().to_string())))
        }
    }
}