use iron::prelude::*;
use iron::status;
use std::error::Error;
use util::{Session, SessionHandler, Json};
use user::common::ExposedSession;

pub struct Handler {}

impl SessionHandler for Handler {
    fn handle_session(&self, session: &mut Session, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Json(ExposedSession{ user_id: session.user_id }))))
    }
}