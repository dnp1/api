use iron::status;
use iron::Response;
use iron::IronResult;

use util::{json};
use std::error::Error;
use uuid::Uuid;
use iron_simple::SimpleHandler;
use super::{AuthenticatedSession, Services};


#[derive(Clone, Serialize, Deserialize, RequestBody)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    new_password: String,
    password: String
}

#[derive(RequestRouteParams)]
pub struct RouteParams {
    user_id: Uuid
}


#[derive(Clone, Serialize, Deserialize)]
struct ResponseBody {
    success: bool
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, AuthenticatedSession, RequestBody);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, session, body) = req;
        let user_id: Uuid = route_params.user_id;

        if session.user_id != user_id {
            return Ok(Response::with((status::Forbidden, "you can only update only your self")))
        }


        let password_match : bool = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => {
                match connection.query(
                    "SELECT set_user_password($1, $2, $3) as password_match",
                    &[&user_id, &body.new_password, &body.password]) {
                    Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                    Ok(rows) => if rows.len() > 0 {
                        rows.get(0).get("password_match")
                    } else {
                        return Ok(Response::with((status::NotFound, "user not found")))
                    }
                }
            }
        };

        let status_code = if password_match {
            status::Ok
        } else {
            status::Unauthorized
        };

        Ok(Response::with((status_code, json(ResponseBody {success: password_match}))))
    }
}