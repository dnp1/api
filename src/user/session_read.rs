use iron::prelude::*;
use iron::status;
use bodyparser;
use std::sync::Arc;
use std::error::Error;
use uuid::Uuid;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Session, SessionHandler};
use serde_json;
use user::common::ExposedSession;

#[derive(Clone, Serialize, Deserialize)]
struct RequestBody {
    email: String,
    password: String
}

pub struct Handler {}

impl SessionHandler for Handler {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        match serde_json::to_string(&ExposedSession{ user_id: session.user_id }) {
            Err(err) => Ok(Response::with((status::InternalServerError, err.description()))),
            Ok(json) => Ok(Response::with((status::Ok, json))),
        }
    }
}