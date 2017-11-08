use router::Router;
use iron::Request;

pub fn get_url_param<'s>(req: &'s Request, name: &'s str) -> &'s str {
    return req.extensions.get::<Router>().unwrap().find(name).unwrap_or("/");
}

use jwt::{encode, decode, Header, Algorithm, Validation};
use jwt::errors::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json;

#[derive(Serialize, Deserialize)]
struct Session {
    session_id: i32,
    user_id: Option<i32>,
    expiration: u64,
}

fn create_session_payload(secret: &str, session: &mut Session) -> Result<String> {
    let expiration = SystemTime::now() + Duration::from_secs(2 ^ 17);
    session.expiration = expiration.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let header = Header::new(Algorithm::HS512);
    return encode(&header, &session, secret.as_ref());
}

fn decode_session_payload(token: &str, secret: &str) -> Option<Session> {
    let result = decode::<Session>(
        token.as_ref(),
        secret.as_ref(),
        &Validation::default());
    match result {
        Err(_) => None,
        Ok(data) => {
            Some(data.claims)
        }
    }
}

trait FileSave {}

