use jwt::{encode, decode, Header, Algorithm, Validation};
use jwt::errors::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use iron::{Request, Response};
use iron::headers::{Cookie, SetCookie, AccessControlAllowCredentials, AccessControlAllowOrigin};


pub const TOKEN_NAME : &'static str = "Authorization";

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub user_id: Option<Uuid>,
    expiration: u64,
}

impl Session {
    pub fn new(session_id: i64) -> Session {
        Session { id: session_id, user_id: None, expiration: 0 }
    }
}

pub struct SessionManager {
    secret: String,
    header: Header,
}


const EXPIRATION_TIME: u64 = 3 * 24 * 60 * 60;

pub fn set_cookie(response: &mut Response, session_payload: &str) {
    response.headers.set(
        SetCookie(vec![
            String::from(format!("{}={};Max-Age={}", TOKEN_NAME, session_payload, 3600))
        ])
    );
}

pub fn set_cors(response: &mut Response) {
    response.headers.set(AccessControlAllowCredentials);
    response.headers.set(AccessControlAllowOrigin::Value("http://localhost:8080".to_owned()));
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
        if let Some(&Cookie(ref cookie)) = req.headers.get() {
            for c in cookie.iter() {
                let prefix = format!("{}=", TOKEN_NAME);
                if c.starts_with(&prefix) {
                    if let Some(value) =  c.get(prefix.len()..) {
                        return self.decode_session_payload(value);
                    }
                }

            }
        }
        None
    }
}