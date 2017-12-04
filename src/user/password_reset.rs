use iron::status;
use iron::Response;
use iron::IronResult;

use super::{Session, Services};
use iron_simple::SimpleHandler;

#[derive(Clone, Serialize, Deserialize, RequestBody)]
struct AccessRecoveryBody {
    email: String,
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (Session);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "TODO:")))
    }
}