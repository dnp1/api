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

#[derive(FromRouteParams)]
pub struct RouteParams {
    user_id: Uuid
}

impl SimpleHandler<RouteParams, Empty, Empty, Empty> for Handler {
    fn handle(&self, req: &SimpleRequest<RouteParams, Empty, Empty, Empty>, session: &mut Session) -> IronResult<Response> {
        let name = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query(
                "SELECT * FROM get_user_name($1)",
                &[&req.route_params.user_id]) {
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