use router::Router;
use iron::request::Request;
mod session;
mod session_handler;
mod storage;
use serde::Serialize;
mod simple_request;

use iron_json_response_modifier::Json;

pub use self::storage::Storage;
pub use self::storage::DiskStorage;
pub use self::session_handler::SessionHandler;
pub use self::session_handler::SessionHandlerBox;
pub use self::session::SessionManager;
pub use self::session::Session;
pub use self::session::TOKEN_NAME;
pub use self::session::set_cookie;
pub use self::simple_request::FromRouteParams;

pub use self::simple_request::SimpleHandler;
pub use self::simple_request::Empty;
pub use self::simple_request::SimpleRequest;
pub use self::simple_request::SimpleHandlerBox;

pub fn json<T>(data : T) -> Json<T>  where T: Serialize {
    Json(data, r#"{"code": "E01", "message": "Data serialization has failed"}"#.as_bytes())
}

pub fn get_url_param_default<'s>(req: &'s Request, name: &'s str) -> &'s str {
    return req.extensions.get::<Router>().unwrap().find(name).unwrap_or("/");
}

pub fn get_url_param<'s>(req: &'s Request, name: &'s str) -> Option<&'s str> {
    return req.extensions.get::<Router>().unwrap().find(name);
}






