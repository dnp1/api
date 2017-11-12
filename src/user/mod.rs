use std::sync::Arc;
use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{SessionManager, SessionHandlerBox};

mod avatar_update;
mod avatar_read;
mod email_read;
mod email_update_request_create;
mod email_update;
mod password_update;
mod name_read;
mod name_update;
mod creation_request_create;
mod create;
mod session_create;
mod password_reset;
mod authenticate;
mod common;



pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, router: &mut Router, sm: Arc<SessionManager>) {
    let db = Arc::new(db);
    let user_avatar_update = avatar_update::Handler { db: db.clone() };
    let user_avatar_read = avatar_read::Handler { db: db.clone() };
    let user_email_read = email_read::Handler { db: db.clone() };
    let user_email_update_request_create = email_update_request_create::Handler { db: db.clone() };
    let user_email_update = email_update::Handler { db: db.clone() };
    let user_password_update = password_update::Handler { db: db.clone() };
    let user_name_read = name_read::Handler { db: db.clone() };
    let user_name_update = name_update::Handler { db: db.clone() };
    let user_creation_request_create = creation_request_create::Handler { db: db.clone() };
    let user_create = create::Handler { db: db.clone() };
    let user_session_create = session_create::Handler { db: db.clone(), sm: sm.clone() };
    let user_password_reset = password_reset::Handler { db: db.clone() };
    let authenticate = authenticate::Handler { db: db.clone() };

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
    router.post("/authenticate", SessionHandlerBox { handler: authenticate, sm: sm.clone() }, "session_authenticate");
}