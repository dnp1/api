use iron::prelude::Response;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::status;

use iron::Handler;
use std::sync::Arc;
use std::error::Error;

use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util;
use util::{SessionManager ,Session};


pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, router: &mut Router, sm : Arc<SessionManager>) {
    let db = Arc::new(db);
    router.put("/user/:user_id/avatar", UserAvatarUpdate { db: db.clone(), sm: sm.clone() }, "user_avatar_update");
    router.get("/user/:user_id/avatar", UserAvatarRead { db: db.clone(), sm: sm.clone() }, "user_avatar_get");
    router.get("/user/:user_id/email", UserEmailRead { db: db.clone(), sm: sm.clone() }, "user_email_read");
    router.post("/user/:user_id/email/update", UserEmailUpdateRequestCreate { db: db.clone(), sm: sm.clone() }, "user_email_update_request_create");
    router.put("/user/:user_id/email", UserEmailUpdate { db: db.clone(), sm: sm.clone() }, "user_email_update");
    router.put("/user/:user_id/password", UserPasswordUpdate { db: db.clone(), sm: sm.clone() }, "user_password_update");
    router.get("/user/:user_id/name", UserNameRead { db: db.clone(), sm: sm.clone() }, "user_name_read");
    router.put("/user/:user_id/name", UserNameUpdate { db: db.clone(), sm: sm.clone() }, "user_name_update");
    router.post("/user/sign-up", UserCreationRequestCreate { db: db.clone(), sm: sm.clone() }, "user_creation_request_create");
    router.post("/user", UserCreate { db: db.clone(), sm: sm.clone() }, "user_create");
    router.post("/session", UserSessionCreate { db: db.clone(), sm: sm.clone() }, "session_create");
    router.post("/user/password-recovery", UserPasswordReset { db: db.clone(), sm: sm.clone() }, "user_password_reset");
}


struct UserAvatarUpdate {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserAvatarUpdate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}

struct UserAvatarRead {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserAvatarRead {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}

struct UserEmailRead {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserEmailRead {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}

struct UserEmailUpdateRequestCreate {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserEmailUpdateRequestCreate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserEmailUpdate {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserEmailUpdate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserPasswordUpdate {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserPasswordUpdate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserNameRead {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserNameRead {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserNameUpdate {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserNameUpdate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserCreationRequestCreate {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserCreationRequestCreate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserCreate {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserCreate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserPasswordReset {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserPasswordReset {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref user_id = util::get_url_param(req, "user_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *user_id)))
        }
    }
}


struct UserSessionCreate {
    db: Arc<Pool<PostgresConnectionManager>>,
    sm: Arc<SessionManager>,
}

impl Handler for UserSessionCreate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let db = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => connection
        };
        let session_id: i64  = match db.query("SELECT create_session() as id", &[]) {
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

