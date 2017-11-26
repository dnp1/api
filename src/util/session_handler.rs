use std::sync::Arc;
use iron::{Request, Response, IronResult, status, Handler};
use util::{SessionManager, Session};
use iron::headers::{SetCookie};
use util::TOKEN_NAME;
use util::{set_cookie};


pub trait SessionHandler {
    fn authenticated(&self) -> bool {
        false
    }
    fn handle(&self, session: &mut Session, req: &mut Request) -> IronResult<Response>;
}

pub struct SessionHandlerBox<T> {
    pub handler: T,
    pub sm: Arc<SessionManager>
}

impl<T> Handler for SessionHandlerBox<T> where T: SessionHandler + Send + Sync + 'static {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut session = match self.sm.get_request_session(req) {
            None => return Ok(Response::with((status::Unauthorized, "You must create a session"))),
            Some(session) => {
                if self.handler.authenticated() {
                    if let None = session.user_id {
                        return Ok(Response::with((status::Unauthorized, "You must authenticate with an user")));
                    }
                }
                session
            }
        };
        match self.handler.handle(&mut session, req) {
            Ok(mut response) => {
                match self.sm.create_session_payload(&mut session) {
                    Err(err) => Ok(Response::with((status::InternalServerError, err.to_string()))),
                    Ok(payload) => {
                        set_cookie(&mut response, &payload);
                        Ok(response)
                    }
                }
            }
            Err(err) => Err(err),
        }
    }
}