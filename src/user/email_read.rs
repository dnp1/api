use iron::status;
use iron::Response;
use iron::IronResult;

use std::error::Error;
use uuid::Uuid;
use postgres::rows;
use util::{json};
use iron_simple::SimpleHandler;
use super::Services;
use super::AuthenticatedSession;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    address: String
}


#[derive(RequestRouteParams)]
pub struct RouteParams {
    user_id: Uuid
}


impl Email {
    pub fn from_row(row: &rows::Row) -> Email {
        return Email {
            address: row.get("address"),
        };
    }
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, AuthenticatedSession);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, _) = req;
        let email = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query(
                "SELECT get_user_email($1) as address",
                &[&route_params.user_id]) {
                Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    Email::from_row(&rows.get(0))
                } else {
                    return Ok(Response::with((status::NotFound, "email for user_id not found")))
                }
            }
        };
        Ok(Response::with((status::Ok, json(email))))
    }
}