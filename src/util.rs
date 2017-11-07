use router::Router;
use iron::Request;

pub fn get_url_param<'s>(req: &'s Request, name: &'s str) -> &'s str {
    return req.extensions.get::<Router>().unwrap().find(name).unwrap_or("/");
}

use frank_jwt::{Header, Payload, Algorithm, encode, decode};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json;

#[derive(Serialize, Deserialize)]
struct Session {
    session_id: i32,
    user_id: Option<i32>,
    expiration: u64,
}

const PAYLOAD_SESSION_KEY: &'static str = "session";

fn create_session_payload(secret: &str, session: &mut Session) -> String {
    let mut payload = Payload::new();

    let expiration = SystemTime::now() + Duration::from_secs(2 ^ 17);
    session.expiration = expiration.duration_since(UNIX_EPOCH).unwrap().as_secs();
    payload.insert(PAYLOAD_SESSION_KEY.to_string(), serde_json::to_string(&session).unwrap());

    let header = Header::new(Algorithm::HS512);
    return encode(header, secret.to_string(), payload.clone());
}

fn decode_session_payload(token: &str, secret: &str) -> Option<Session> {
    let result = decode(
        token.to_string(),
        secret.to_string(),
        Algorithm::HS512);
    match result {
        Err(_) => None,
        Ok((_, data)) => {
            match data.get(PAYLOAD_SESSION_KEY) {
                None => None,
                Some(s) => serde_json::from_str(&s).unwrap()
            }
        }
    }
}
