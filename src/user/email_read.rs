use iron::prelude::*;
use iron::status;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use std::error::Error;
use uuid::Uuid;
use postgres::rows;
use util::{Session, SimpleHandler, SimpleRequest, Empty, FromRouteParams, json};
use std::str::FromStr;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    address: String
}


#[derive(FromRouteParams)]
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

impl SimpleHandler<RouteParams, Empty, Empty, Empty> for Handler {
    fn handle(&self, req: &SimpleRequest<RouteParams, Empty, Empty, Empty>, session: &mut Session) -> IronResult<Response> {
        let email = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query(
                "SELECT get_user_email($1) as address",
                &[&req.route_params.user_id]) {
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