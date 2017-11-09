use iron::prelude::Response;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::status;

use iron::Handler;
use std::sync::Arc;
use std::error::Error;
use uuid::Uuid;
use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{SessionManager, Session, SessionHandler, SessionHandlerBox};

pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, router: &mut Router, sm: Arc<SessionManager>) {
    let db = Arc::new(db);
    let user_avatar_update = UserAvatarUpdate { db: db.clone() };
    let user_avatar_read = UserAvatarRead { db: db.clone() };
    let user_email_read = UserEmailRead { db: db.clone() };
    let user_email_update_request_create = UserEmailUpdateRequestCreate { db: db.clone() };
    let user_email_update = UserEmailUpdate { db: db.clone() };
    let user_password_update = UserPasswordUpdate { db: db.clone() };
    let user_name_read = UserNameRead { db: db.clone() };
    let user_name_update = UserNameUpdate { db: db.clone() };
    let user_creation_request_create = UserCreationRequestCreate { db: db.clone() };
    let user_create = UserCreate { db: db.clone() };
    let user_session_create = SessionCreate { db: db.clone(), sm: sm.clone() };
    let user_password_reset = UserPasswordReset { db: db.clone() };
    router.put("/user/:user_id/avatar", SessionHandlerBox { handler: user_avatar_update, sm: sm.clone() }, "user_avatar_update");
    router.get("/user/:user_id/avatar", SessionHandlerBox { handler: user_avatar_read, sm: sm.clone() }, "user_avatar_get");
    router.get("/user/:user_id/email", SessionHandlerBox { handler: user_email_read, sm: sm.clone() }, "user_email_read");
    router.post("/user/:user_id/email/update", SessionHandlerBox { handler: user_email_update_request_create, sm: sm.clone() }, "user_email_update_request_create");
    router.put("/user/:user_id/email", SessionHandlerBox { handler: user_email_update, sm: sm.clone() }, "user_email_update");
    router.put("/user/:user_id/password", SessionHandlerBox { handler: user_password_update, sm: sm.clone() }, "user_password_update");
    router.get("/user/:user_id/name", SessionHandlerBox { handler: user_name_read, sm: sm.clone() }, "user_name_read");
    router.put("/user/:user_id/name", SessionHandlerBox { handler: user_name_update, sm: sm.clone() }, "user_name_update");
    router.post("/user/sign-up", SessionHandlerBox { handler: user_creation_request_create, sm: sm.clone() }, "user_creation_request_create");
    router.post("/user", SessionHandlerBox { handler: user_create, sm: sm.clone() }, "user_create");
    router.post("/user/password-recovery", SessionHandlerBox { handler: user_password_reset, sm: sm.clone() }, "user_password_reset");
    router.post("/session", user_session_create, "session_create");
}


struct UserAvatarUpdate {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for UserAvatarUpdate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}

struct UserAvatarRead {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserAvatarRead {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}

struct UserEmailRead {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserEmailRead {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}

struct UserEmailUpdateRequestCreate {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserEmailUpdateRequestCreate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserEmailUpdate {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserEmailUpdate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserPasswordUpdate {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserPasswordUpdate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserNameRead {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserNameRead {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserNameUpdate {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserNameUpdate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserCreationRequestCreate {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserCreationRequestCreate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserCreate {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserCreate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserPasswordReset {
    db: Arc<Pool<PostgresConnectionManager>>
}

impl SessionHandler for UserPasswordReset {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct SessionCreate {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for SessionCreate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let db = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => connection
        };
        let session_id: i64 = match db.query("SELECT create_session() as id", &[]) {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(rows) => {
                let row = &rows.get(0);
                row.get("id")
            }
        };

        if let Ok(session) = self.sm.create_session_payload(&mut Session::new(session_id)) {
            Ok(Response::with((status::Ok, session)))
        } else {
            Ok(Response::with((status::ServiceUnavailable, "")))
        }
    }
}

struct Authenticate {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for Authenticate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let user_id: Uuid = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query(
                "SELECT authenticate($1, $2, $3, $4) as ok",
                &[]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => {
                    rows.get(0).get("")
                }
            },
        };
        session.user_id = Some(user_id);
        Ok(Response::with((status::ServiceUnavailable, user_id.simple().to_string())))
    }
}