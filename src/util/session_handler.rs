use std::sync::Arc;
use iron::{Request, Response, IronResult, status, Handler};
use iron::headers::Authorization;
use util::{SessionManager, Session};

pub trait SessionHandler {
    fn authenticated(&self) -> bool {
        false
    }
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response>;
}

pub struct SessionHandlerBox<T> {
    pub handler: T,
    pub sm: Arc<SessionManager>
}

impl<T> Handler for SessionHandlerBox<T> where T: SessionHandler + Send + Sync + 'static {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut session = match self.sm.get_request_session(req) {
            None => {
                let result = Response::with((status::Unauthorized, "You must create a session"));
                return Ok(result);
            }
            Some(session) => {
                if self.handler.authenticated() {
                    if let None = session.user_id {
                        let result = Response::with((status::Unauthorized, "You must authenticate with an user"));
                        return Ok(result);
                    }
                }
                session
            }
        };
        match self.handler.handle_session(&mut session, req) {
            Ok(mut response) => {
                match self.sm.create_session_payload(&mut session) {
                    Err(err) => Ok(Response::with((status::InternalServerError, err.to_string()))),
                    Ok(payload) => {
                        response.headers.set(Authorization(payload));
                        Ok(response)
                    }
                }
            }
            Err(err) => Err(err),
        }
    }
}