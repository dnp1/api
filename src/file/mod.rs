use std::sync::Arc;
use std::error::Error;

use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::simple_adapter::JsonErrorTransformer;

use util::{Storage};
use util::session_manager::SessionManager;
use iron_simple::SimpleHandler;

mod create;
mod read;
mod delete;

pub type Session = ::util::Session;
pub type AuthenticatedSession = ::util::AuthenticatedSession;

use util::DiskStorage;

pub struct FileServices {
    pub db: Pool<PostgresConnectionManager>,
    pub session_manager: SessionManager,
    pub storage: DiskStorage,
}

pub type Services = Arc<FileServices>;

impl ::util::session::SessionManager for Services {
    fn get_session_manager(&self) -> &::util::session_manager::SessionManager {
        &self.session_manager
    }
}

pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, r: &'s mut Router, session_manager: SessionManager, storage: DiskStorage) {
    let services = Arc::from(FileServices {
        db,
        session_manager,
        storage,
    });

    let file_read = read::Handler.handler(services.clone(), JsonErrorTransformer);
    let file_create = create::Handler.handler(services.clone(), JsonErrorTransformer);
    let file_delete = delete::Handler.handler(services.clone(), JsonErrorTransformer);
    r.post("/file", file_read, "file_create");
    r.get("/file/:file_id", file_create, "file_read");
    r.delete("/file/:file_id", file_delete, "file_delete");
}
