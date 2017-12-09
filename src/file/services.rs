use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Storage};
use util::session_manager::SessionManager;

pub struct FileServices<T: Storage> {
    pub db: Pool<PostgresConnectionManager>,
    pub session_manager: SessionManager,
    pub storage: T,
}
