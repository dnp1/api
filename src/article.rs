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

struct ArticleList { db: Arc<Pool<PostgresConnectionManager>> }

pub fn register_handlers<'s>(db: Pool<PostgresConnectionManager>, router: &mut Router) {
    let db = Arc::new(db);
    router.get("/article", ArticleList { db: db.clone() }, "article_list");
    router.get("/article/:article_id", ArticleRead { db: db.clone() }, "article_read");
    router.get("/article/:article_id/tag", ArticleTagList { db: db.clone() }, "article_tag_list");
    router.get("/article/:article_id/comment", ArticleCommentList { db: db.clone() }, "article_comment_list");
    router.get("/article/:article_id/comment/:comment_id", ArticleCommentRead { db: db.clone() }, "article_comment_read");
    router.post("/article/:article_id/comment", ArticleCommentCreate { db: db.clone() }, "article_comment_create");
}

impl Handler for ArticleList {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
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

struct ArticleRead { db: Arc<Pool<PostgresConnectionManager>> }

impl Handler for ArticleRead {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}


struct ArticleTagList { db: Arc<Pool<PostgresConnectionManager>> }

impl Handler for ArticleTagList {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}

struct ArticleCommentRead { db: Arc<Pool<PostgresConnectionManager>> }

impl Handler for ArticleCommentRead {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}

struct ArticleCommentList { db: Arc<Pool<PostgresConnectionManager>> }

impl Handler for ArticleCommentList {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        let ref comment_id = util::get_url_param(req, "comment_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}

struct ArticleCommentCreate { db: Arc<Pool<PostgresConnectionManager>> }

impl Handler for ArticleCommentCreate {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref article_id = util::get_url_param(req, "article_id");
        let ref comment_id = util::get_url_param(req, "comment_id");
        match self.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
        }
    }
}

