use iron::prelude::*;
use iron::status;
use std::error::Error;
use util::{Session, SessionHandler, json};
use user::common::ExposedSession;

pub struct Handler {}

impl SessionHandler for Handler {
    fn handle(&self, session: &mut Session, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, json(ExposedSession{ user_id: session.user_id }))))
    }
}