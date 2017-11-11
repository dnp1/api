use std::sync::Arc;
use std::error::Error;

use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{SessionManager, SessionHandlerBox};

mod comment_create;
mod comment_list;
mod comment_read;
mod list;
mod read;
mod tag_list;
mod common;

pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, router: &mut Router, sm: Arc<SessionManager>) {
    let db = Arc::new(db);

    let article_list = list::Handler { db: db.clone() };
    let article_read = read::Handler { db: db.clone() };
    let article_tag_list = tag_list::Handler { db: db.clone() };
    let article_comment_list = comment_list::Handler { db: db.clone() };
    let article_comment_read = comment_read::Handler { db: db.clone() };
    let article_comment_create = comment_create::Handler { db: db.clone() };

    router.get("/article", SessionHandlerBox { handler: article_list, sm: sm.clone() }, "article_list");
    router.get("/article/:article_id", SessionHandlerBox { handler: article_read, sm: sm.clone() }, "article_read");
    router.get("/article/:article_id/tag", SessionHandlerBox { handler: article_tag_list, sm: sm.clone() }, "article_tag_list");
    router.get("/article/:article_id/comment", SessionHandlerBox { handler: article_comment_list, sm: sm.clone() }, "article_comment_list");
    router.get("/article/:article_id/comment/:comment_id", SessionHandlerBox { handler: article_comment_read, sm: sm.clone() }, "article_comment_read");
    router.post("/article/:article_id/comment", SessionHandlerBox { handler: article_comment_create, sm: sm.clone() }, "article_comment_create");
}











