use r2d2_postgres::{PostgresConnectionManager};
use r2d2::Pool;
use util::session_manager::SessionManager;
use std::sync::Arc;

pub struct CommonServices {
    pub db: Pool<PostgresConnectionManager>,
    pub session_manager: SessionManager,
}

pub type Services = Arc<CommonServices>;

impl ::util::session::SessionManager for Services {
    fn get_session_manager(&self) -> &::util::session_manager::SessionManager {
        &self.session_manager
    }
}
