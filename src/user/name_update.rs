use iron::status;
use iron::Response;
use iron::IronResult;

use util::{json};
use std::error::Error;
use uuid::Uuid;

use super::{AuthenticatedSession, Services};
use iron_simple::SimpleHandler;


#[derive(Clone, Serialize, Deserialize, RequestBody)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    given_name: String,
    family_name: String,
    password: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponseBody {
    success: bool
}

#[derive(RequestRouteParams)]
pub struct RouteParams {
    user_id: Uuid
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, AuthenticatedSession, RequestBody);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, session, body) = req;

        if session.user_id != route_params.user_id {
            return Ok(Response::with((status::Forbidden, "you can only update only your self")))
        }


        let password_match : bool = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => {
                match connection.query(
                    "SELECT set_user_name($1, $2, $3, $4) as password_match",
                    &[&route_params.user_id, &body.given_name, &body.family_name, &body.password]) {
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