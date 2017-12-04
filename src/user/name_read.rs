use iron::status;
use iron::Response;
use iron::IronResult;

use std::error::Error;
use uuid::Uuid;
use postgres::rows;
use util::{json};
use iron_simple::SimpleHandler;
use super::{Session, Services};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    given_name: String,
    family_name: String,
}

impl Name {
    pub fn from_row(row: &rows::Row) -> Name {
        Name {
            given_name: row.get("given_name"),
            family_name: row.get("family_name")
        }
    }
}

#[derive(RequestRouteParams)]
pub struct RouteParams {
    user_id: Uuid
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, Session);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, _) = req;

        let name = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query(
                "SELECT * FROM get_user_name($1)",
                &[&route_params.user_id]) {
                Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    Name::from_row(&rows.get(0))
                } else {
                    return Ok(Response::with((status::NotFound, "user not found")));
                }
            }
        };
        Ok(Response::with((status::Ok, json(name))))
    }
}