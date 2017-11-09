use iron::prelude::Response;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::status;
//use iron::IronError;

use iron::Handler;
use std::sync::Arc;
use std::error::Error;

use router::Router;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use util;
use util::{SessionManager, Session, SessionHandler, SessionHandlerBox};

pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, router: &mut Router, sm: Arc<SessionManager>) {
    let db = Arc::new(db);

    let article_list = ArticleList { db: db.clone() };
    let article_read = ArticleRead { db: db.clone() };
    let article_tag_list = ArticleTagList { db: db.clone() };
    let article_comment_list = ArticleCommentList { db: db.clone() };
    let article_comment_read = ArticleCommentRead { db: db.clone() };
    let article_comment_create = ArticleCommentCreate { db: db.clone() };

    router.get("/article", SessionHandlerBox { handler: article_list, sm: sm.clone() }, "article_list");
    router.get("/article/:article_id", SessionHandlerBox { handler: article_read, sm: sm.clone() }, "article_read");
    router.get("/article/:article_id/tag", SessionHandlerBox { handler: article_tag_list, sm: sm.clone() }, "article_tag_list");
    router.get("/article/:article_id/comment", SessionHandlerBox { handler: article_comment_list, sm: sm.clone() }, "article_comment_list");
    router.get("/article/:article_id/comment/:comment_id", SessionHandlerBox { handler: article_comment_read, sm: sm.clone() }, "article_comment_read");
    router.post("/article/:article_id/comment", SessionHandlerBox { handler: article_comment_create, sm: sm.clone() }, "article_comment_create");
}

struct ArticleList {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for ArticleList {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => {
                //                req.get_ref()
                connection.query("SELECT * FROM article_list($1, 2)", &[]);
                Ok(Response::with((status::Ok, "")))
            }
        }
    }
}

struct ArticleRead {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for ArticleRead {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}


struct ArticleTagList {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for ArticleTagList {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}

struct ArticleCommentRead {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for ArticleCommentRead {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}

struct ArticleCommentList {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for ArticleCommentList {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        let ref comment_id = util::get_url_param(req, "comment_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}

struct ArticleCommentCreate {
    db: Arc<Pool<PostgresConnectionManager>>,
}

impl SessionHandler for ArticleCommentCreate {
    fn handle_session(&self, session: &mut Session, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        let ref comment_id = util::get_url_param(req, "comment_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}

