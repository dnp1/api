use iron::status;
use iron::Response;
use iron::IronResult;
use std::error::Error;
use uuid::Uuid;
use util::{json, set_session_cookie};
use user::common::ExposedSession;
use iron_simple::SimpleHandler;

use super::{Services, Session};

#[derive(Clone, Serialize, Deserialize, RequestBody)]
pub struct Body {
    email: String,
    password: String
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (Body, Session);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (body, session) = req;

        let user_id: Uuid = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query(
                "SELECT authenticate($1, $2, $3, '192.168.43.37') as ok",
                &[&session.id, &body.email, &body.password]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => {
                    if rows.len() > 0 {
                        rows.get(0).get("ok")
                    } else {
                        return Ok(Response::with((status::Unauthorized, "")))
                    }
                }
            },
        };

        let mut session = session.clone();
        session.user_id = Some(user_id);

        if let Ok(session) = services.session_manager.create_session_payload(&mut session) {
            let mut response = Response::with((status::Ok, json(&ExposedSession{user_id: Some(user_id)})));
            set_session_cookie(&mut response, &session);
            Ok(response)
        } else {
            Ok(Response::with((status::ServiceUnavailable, "TODO")))
        }
    }
}