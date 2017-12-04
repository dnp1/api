use iron::prelude::*;
use iron::status;
use util::{json};
use user::common::ExposedSession;

use iron_simple::SimpleHandler;
use super::{Services, Session};

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (Session,);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (session,) = req;

        Ok(Response::with((status::Ok, json(ExposedSession{ user_id: session.user_id }))))
    }
}