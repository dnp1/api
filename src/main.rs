extern crate iron;
extern crate router;

use std::time::Duration;

use iron::prelude::*;
use iron::status;
use iron::Timeouts;
use router::Router;

fn article_list(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn article_read(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn article_archive(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn article_tag_list(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn article_comment_list(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn article_comment_read(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn article_comment_create(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn session_create(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn file_create(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_avatar_update(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_avatar_get(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_email_read(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_email_update_request_create(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_email_update(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_password_update(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_name_read(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_name_update(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_creation_request_create(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_create(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn user_password_reset(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}


fn main() {
    let mut router = Router::new();
    router.get("/article", article_list, "article_list");
    router.get("/article/:articleId", article_read, "article_read");
    router.get("/article/archive", article_archive, "article_archive");
    router.get("/article/:articleId/tag", article_tag_list, "article_tag_list");
    router.get("/article/:articleId/comment", article_comment_list, "article_comment_list");
    router.get("/article/:articleId/comment/:commentId", article_comment_read, "article_comment_read");
    router.post("/article/:articleId/coment", article_comment_create, "article_comment_create");
    router.post("/session", session_create, "session_create");
    router.post("/file", file_create, "file_create");
    router.put("/user/:userId/avatar", user_avatar_update, "user_avatar_update");
    router.get("/user/:userId/avatar", user_avatar_get, "user_avatar_get");
    router.get("/user/:userId/email", user_email_read, "user_email_read");
    router.post("/user/:userId/email/update", user_email_update_request_create, "user_email_update_request_create");
    router.put("/user/:userId/email", user_email_update, "user_email_update");
    router.put("/user/:userId/password", user_password_update, "user_password_update");
    router.get("/user/:userId/name", user_name_read, "user_name_read");
    router.put("/user/:userId/name", user_name_update, "user_name_update");
    router.post("/user/sign-up", user_creation_request_create, "user_creation_request_create");
    router.post("/user", user_create, "user_create");
    router.post("/password-recovery", user_password_reset, "user_password_reset");

    let mut iron = Iron::new(router);

    iron.threads = 8;
    iron.timeouts = Timeouts {
        keep_alive: Some(Duration::from_secs(10)),
        read: Some(Duration::from_secs(10)),
        write: Some(Duration::from_secs(10))
    };

    iron.http("localhost:3000").unwrap();
}

