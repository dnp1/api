use iron::prelude::*;
use iron::status;
use std::error::Error;
use util::{Session, SessionHandler};
use serde_json;
use user::common::ExposedSession;

pub struct Handler {}

impl SessionHandler for Handler {
    fn handle_session(&self, session: &mut Session, _: &mut Request) -> IronResult<Response> {
        match serde_json::to_string(&ExposedSession{ user_id: session.user_id }) {
            Err(err) => Ok(Response::with((status::InternalServerError, err.description()))),
            Ok(json) => Ok(Response::with((status::Ok, json))),
        }
    }
}