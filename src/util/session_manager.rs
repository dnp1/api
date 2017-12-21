use iron::{Request, Response};
use iron::headers::{Cookie, SetCookie};
use iron::headers::CookieIter;
use jwt::{encode, decode, Header, Algorithm, Validation};
use jwt::errors::Result;
use super::session::Session;
use iron::headers;

pub const TOKEN_NAME: &'static str = "Authorization";

#[derive(Clone)]
pub struct SessionManager {
    secret: String,
    header: Header,
}


pub fn set_session_cookie(response: &mut Response, session_payload: &str) {
    response.headers.set(
        SetCookie(vec![
            String::from(format!("{}={};Max-Age={};Path=/", TOKEN_NAME, session_payload, 3600))
        ])
    );
}


impl SessionManager {
    pub fn new(secret: &str) -> SessionManager {
        let header = Header::new(Algorithm::HS512);
        SessionManager {
            header,
            secret: secret.to_owned()
        }
    }
    pub fn create_session_payload(&self, session: &mut Session) -> Result<String> {
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
        match req.headers.get::<Cookie>() {
            None => return None,
            Some(cookie) => match cookie.get(TOKEN_NAME) {
                Some(value) => {
                    return self.decode_session_payload(value);
                }
                None => None,
            }
        };
        None
    }
}