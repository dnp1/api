use std::sync::Arc;
use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::session_manager::SessionManager;

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
mod session_read;
mod password_reset;
mod authenticate;
mod common;

use iron_simple::SimpleHandler;

pub type Services = ::util::services::Services;
pub type Session = ::util::Session;
pub type AuthenticatedSession = ::util::AuthenticatedSession;

pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, router: &mut Router, session_manager: SessionManager) {
    use util::simple_adapter::JsonErrorTransformer;
    use util::services::CommonServices;

    let services = Arc::from(CommonServices {
        db,
        session_manager
    });

    let user_avatar_update = avatar_update::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let user_avatar_read = avatar_read::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let user_email_read = email_read::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let user_email_update_request_create = email_update_request_create::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let user_email_update = email_update::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let user_password_update = password_update::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let user_name_read = name_read::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let user_name_update = name_update::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let user_creation_request_create = creation_request_create::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let user_create = create::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let session_create = session_create::Handler{services: services.clone()};

    let user_password_reset = password_reset::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let authenticate = authenticate::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    let session_read = session_read::Handler.handler(
        services.clone(),
        JsonErrorTransformer
    );
    router.put("/user/:user_id/avatar", user_avatar_update, "user_avatar_update");
    router.get("/user/:user_id/avatar", user_avatar_read, "user_avatar_get");
    router.get("/user/:user_id/email", user_email_read, "user_email_read");
    router.post("/user/:user_id/email/update", user_email_update_request_create, "user_email_update_request_create");
    router.put("/user/:user_id/email",user_email_update, "user_email_update");
    router.put("/user/:user_id/password", user_password_update, "user_password_update");
    router.get("/user/:user_id/name", user_name_read, "user_name_read");
    router.put("/user/:user_id/name", user_name_update, "user_name_update");
    router.post("/user/sign-up", user_creation_request_create, "user_creation_request_create");
    router.post("/user", user_create, "user_create");
    router.post("/user/password-recovery", user_password_reset, "user_password_reset");
    router.post("/session", session_create, "session_create");
    router.get("/session", session_read, "session_read");
    router.post("/authenticate", authenticate, "session_authenticate");
}