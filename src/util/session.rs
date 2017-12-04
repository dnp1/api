use uuid::Uuid;
use iron::Request;
use iron_simple::{SimpleResult, FromIronRequest};
use iron_simple::{SimpleError, ClientError, ServerError};

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: i64,
    pub user_id: Option<Uuid>,
}

impl Session {
    pub fn new(session_id: i64) -> Session {
        Session { id: session_id, user_id: None}
    }
}


pub struct AuthenticatedSession {
    pub id: i64,
    pub user_id: Uuid,
}


pub trait SessionManager: Sync + Send + 'static {
  fn get_session_manager(&self) -> &::util::session_manager::SessionManager;
}


impl <T: SessionManager> FromIronRequest<T> for Session {
    fn from_request<'a>(req: &mut Request, services: &T) -> SimpleResult<Self> {
        match services.get_session_manager().get_request_session(req) {
            None => Err(SimpleError::Client(ClientError::MissingSession("no session found".to_owned()))),
            Some(val) => Ok(val),
        }
    }
}

impl <T: SessionManager> FromIronRequest<T> for AuthenticatedSession {
    fn from_request<'a>(req: &mut Request, services: &T) -> SimpleResult<Self> {
        match services.get_session_manager().get_request_session(req) {
            None => return Err(SimpleError::Client(ClientError::MissingSession("no session found".to_owned()))),
            Some(session) => match session.user_id {
                None => return Err(SimpleError::Client(ClientError::InvalidSession("For access this you must be authenticated".to_owned()))),
                Some(user_id) => Ok(
                    AuthenticatedSession{
                        id: session.id,
                        user_id,
                    }
                )
            },
        }
    }
}

