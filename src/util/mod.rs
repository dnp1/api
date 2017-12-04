use router::Router;
use iron::request::Request;
use serde::Serialize;

pub mod session;
pub mod session_manager;
pub mod storage;
pub mod simple_adapter;
pub mod services;

use iron_json_response_modifier::Json;

pub use self::storage::Storage;
pub use self::storage::DiskStorage;
pub use self::session::Session;
pub use self::session::AuthenticatedSession;

pub use self::session_manager::set_cookie;


pub fn json<T>(data : T) -> Json<T>  where T: Serialize {
    Json(data, r#"{"code": "E01", "message": "Data serialization has failed"}"#.as_bytes())
}



