use iron::status;
use iron::Response;
use iron::IronResult;
use util::{json};
use std::error::Error;
use uuid::Uuid;
use iron_simple::{SimpleHandler};
use super::{Session, Services};
pub struct Handler;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseBody {
    file_id: Uuid
}

#[derive(RequestRouteParams)]
pub struct RouteParams {
    user_id: Uuid,
}

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, Session);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, _) = req;

        let file_id: Uuid = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query(
                "SELECT get_user_avatar($1) as file_id",
                &[&route_params.user_id]) {
                Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    rows.get(0).get("file_id")
                } else {
                    return Ok(Response::with((status::NotFound, "avatar not found for user_id")))
                }
            }
        };
        Ok(Response::with((status::Ok, json(ResponseBody {file_id }))))
    }
}
