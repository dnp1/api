use iron::prelude::*;
use iron::status;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Session, SessionHandler};

#[derive(Clone, Serialize, Deserialize)]
struct AccessRecoveryBody {
    email: String,
}

pub struct UserPasswordReset {
    pub db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserPasswordReset {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "TODO:")))
    }
}