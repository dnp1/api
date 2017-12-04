use iron::status;
use iron::Response;
use iron::IronResult;
use std::error::Error;
use iron_simple::SimpleHandler;
use super::Services;

#[derive(Clone, Serialize, Deserialize)]
struct UserCreateBody {
    email: String,
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = ();

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        match services.db.get() {
            Err(err) => Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => Ok(Response::with((status::Ok, "")))
        }
    }
}