mod comment_create;
mod comment_list;
mod comment_read;
mod list;
mod read;
mod tag_list;
mod common;
mod read_content;
mod comment_read_content;

use std::sync::Arc;

use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use util::services::CommonServices;
use util::simple_adapter::JsonErrorTransformer;
use util::session_manager::SessionManager;

use iron_simple::SimpleHandler;


pub type Services = ::util::services::Services;
pub type Session = ::util::Session;
pub type AuthenticatedSession = ::util::AuthenticatedSession;



pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, router: &mut Router, session_manager: SessionManager) {
    let services = Arc::from(CommonServices {
        db,
        session_manager
    });

    let article_list = list::Handler.handler(
        services.clone(),
        JsonErrorTransformer);
    let article_read = read::Handler.handler(
        services.clone(),
        JsonErrorTransformer);
    let article_read_content = read_content::Handler.handler(
        services.clone(),
        JsonErrorTransformer);
    let article_tag_list = tag_list::Handler.handler(
        services.clone(),
        JsonErrorTransformer);
    let article_comment_list = comment_list::Handler.handler(
        services.clone(),
        JsonErrorTransformer);
    let article_comment_read = comment_read::Handler.handler(
        services.clone(),
        JsonErrorTransformer);
    let article_comment_create = comment_create::Handler.handler(
        services.clone(),
        JsonErrorTransformer);
    let article_comment_read_content = comment_read_content::Handler.handler(
        services.clone(),
        JsonErrorTransformer);



    router.get("/article", article_list, "article_list");
    router.get("/article/:article_id", article_read, "article_read");
    router.get("/article/:article_id/content", article_read_content, "article_comment_create_content");
    router.get("/article/:article_id/tag", article_tag_list, "article_tag_list");
    router.get("/article/:article_id/comment", article_comment_list, "article_comment_list");
    router.get("/article/:article_id/comment/:comment_id", article_comment_read, "article_comment_read");
    router.get("/article/:article_id/comment/:comment_id/content", article_comment_create, "article_comment_read_content");
    router.post("/article/:article_id/comment", article_comment_read_content, "article_comment_create");
}











