use std::sync::Arc;
use std::error::Error;

use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use util::{SessionManager, SessionHandlerBox, SimpleHandlerBox, Storage};

mod create;
mod read;
mod delete;

pub fn register_handlers<'s, T>(db: Pool<PostgresConnectionManager>, r: &'s mut Router, sm: Arc<SessionManager>, storage: Arc<T>) where T : Storage {
    let db = Arc::new(db);
    let file_read = read::Handler { db: db.clone(), storage: storage.clone() };
    let file_create = create::Handler { db: db.clone(), storage: storage.clone() };
    let file_delete = delete::Handler { db: db.clone() };
    r.post("/file", SessionHandlerBox { handler: file_create, sm: sm.clone() }, "file_create");
    r.get("/file/:file_id", SimpleHandlerBox::new( file_read,  sm.clone() ), "file_read");
    r.delete("/file/:file_id", SimpleHandlerBox::new(file_delete,  sm.clone() ), "file_delete");
}
