use router::Router;
use iron::{Request, Response, IronResult, status, Handler};
use iron::headers::Authorization;

pub fn get_url_param<'s>(req: &'s Request, name: &'s str) -> &'s str {
    return req.extensions.get::<Router>().unwrap().find(name).unwrap_or("/");
}

use jwt::{encode, decode, Header, Algorithm, Validation};
use jwt::errors::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ::std::sync::Arc;
use serde_json;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Session {
    session_id: i64,
    pub user_id: Option<Uuid>,
    expiration: u64,
}

impl Session {
    pub fn new(session_id: i64) -> Session {
        Session { session_id, user_id: None, expiration: 0 }
    }
}

pub struct SessionManager {
    secret: String,
    header: Header,
}


const EXPIRATION_TIME: u64 = 3 * 24 * 60 * 60;

impl SessionManager {
    pub fn new(secret: &str) -> SessionManager {
        let header = Header::new(Algorithm::HS512);
        SessionManager {
            header,
            secret: secret.to_owned()
        }
    }
    pub fn create_session_payload(&self, session: &mut Session) -> Result<String> {
        let expiration = SystemTime::now() + Duration::from_secs(EXPIRATION_TIME);
        session.expiration = expiration.duration_since(UNIX_EPOCH).unwrap().as_secs();
        return encode(&self.header, &session, self.secret.as_ref());
    }
    fn decode_session_payload(&self, token: &str) -> Option<Session> {
        let result = decode::<Session>(
            token.as_ref(),
            self.secret.as_ref(),
            &Validation::default());
        match result {
            Err(_) => None,
            Ok(data) => {
                Some(data.claims)
            }
        }
    }

    pub fn get_request_session(&self, req: &Request) -> Option<Session> {
        let authorization: Option<&Authorization<String>> = req.headers.get();
        match authorization {
            None => None,
            Some(value) => self.decode_session_payload(value),
        }
    }
}

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
            None => return Ok(Response::with((status::Unauthorized, ""))),
            Some(session) => {
                if self.handler.authenticated() {
                    if let None = session.user_id {
                        return Ok(Response::with((status::Forbidden, "")));
                    }
                }
                session
            }
        };
        self.handler.handle_session(&mut session, req)
    }
}