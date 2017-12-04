use iron::Response;
use iron::IronResult;

use iron_simple::SimpleHandler;

use super::{AuthenticatedSession, Services};


pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (AuthenticatedSession);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
//        let ref article_id = util::get_url_param_default(req, "article_id");
//        let ref comment_id = util::get_url_param_default(req, "comment_id");
//        match self.db.get() {
//            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
//            Ok(connection) => Ok(Response::with((status::Ok, *article_id)))
//        }
        Ok(Response::with("banana"))
    }
}