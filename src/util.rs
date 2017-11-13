use router::Router;
use iron::{Request, Response, IronResult, status, Handler};
use iron::headers::Authorization;
use jwt::errors::Error;
use std::io::{Read, Write, BufWriter, BufRead};
use std::fs::File;
use std::path::PathBuf;


pub fn get_url_param_default<'s>(req: &'s Request, name: &'s str) -> &'s str {
    return req.extensions.get::<Router>().unwrap().find(name).unwrap_or("/");
}

pub fn get_url_param<'s>(req: &'s Request, name: &'s str) -> Option<&'s str> {
    return req.extensions.get::<Router>().unwrap().find(name);
}

use jwt::{encode, decode, Header, Algorithm, Validation};
use jwt::errors::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use uuid::Uuid;
use std::error;
use std::result;

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


type StorageResult = result::Result<usize, StorageError>;

#[derive(Debug)]
pub struct StorageError {}
impl error::Error for StorageError{
    fn description(&self) -> &str {
       "Could not save file"
    }
}

use std::fmt;
impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

pub trait Storage: Send + Sync + 'static {
    fn save<I>(&self, filename: &str, content: &mut I) -> StorageResult where I : BufRead;
}


pub struct DiskStorage {
    directory: PathBuf
}

impl DiskStorage {
    pub fn new(directory: &str) -> DiskStorage {
        DiskStorage{directory: PathBuf::from(directory)}
    }
}

impl Storage for DiskStorage {
    fn save<I>(&self, filename: &str, content:&mut I) -> StorageResult   where I : BufRead {
        let file_path = self.directory.join(filename);
        let file = match File::create("foo.txt") {
            Err(_) => return Err(StorageError{}),
            Ok(file) => file,

        };
        let mut written: usize = 0;
        let mut buffer = BufWriter::new(file);
        loop {
            let read = match content.fill_buf() {
                Err(err) => return Err(StorageError{}),
                Ok(bytes) => match buffer.write_all(bytes) {
                    Err(err) => return Err(StorageError{}),
                    Ok(_) => {
                        written += bytes.len();
                        bytes.len()
                    },
                },
            };
            if read == 0 {
                break;
            }
            content.consume(read);
        }
        match buffer.flush() {
            Err(_) => Err(StorageError{}),
            Ok(_) => Ok(written),
        }

    }
}
