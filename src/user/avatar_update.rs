use iron::status;
use iron::Response;
use iron::IronResult;

use std::error::Error;
use util::{json};
use uuid::Uuid;
use iron_simple::SimpleHandler;

use super::{AuthenticatedSession, Services};

#[derive(Clone, Serialize, Deserialize, RequestBody)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    file_id: Uuid,
    password: String
}

#[derive(RequestRouteParams)]
pub struct RequestRouteParams {
    user_id: Uuid
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Resp {
    success: bool
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RequestRouteParams, AuthenticatedSession, RequestBody,);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, session, body) = req;

        if session.user_id != route_params.user_id {
            return Ok(Response::with((status::Forbidden, "you can only update only your self")))
        }

        let password_match : bool = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => {
                match connection.query(
                    "SELECT set_user_avatar($1, $2, $3) as password_match",
                    &[&route_params.user_id, &body.file_id, &body.password]) {
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
        Ok(Response::with((status_code, json(Resp{success: password_match}))))
    }
}