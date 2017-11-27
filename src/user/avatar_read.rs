use iron::prelude::*;
use iron::status;

use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use util::{Session, SimpleHandler, SimpleRequest, Empty, FromRouteParams, json};
use std::str::FromStr;
use std::error::Error;
use uuid::Uuid;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Avatar {
    file_id: Uuid
}

#[derive(FromRouteParams)]
pub struct RouteParams {
    user_id: Uuid,
}

impl SimpleHandler<RouteParams, Empty, Empty, Empty> for Handler {
    fn handle(&self, req: &SimpleRequest<RouteParams, Empty, Empty, Empty>, session: &mut Session) -> IronResult<Response> {
        let file_id: Uuid = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query(
                "SELECT get_user_avatar($1) as file_id",
                &[&req.route_params.user_id]) {
                Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    rows.get(0).get("file_id")
                } else {
                    return Ok(Response::with((status::NotFound, "avatar not found for user_id")))
                }
            }
        };
        Ok(Response::with((status::Ok, json(Avatar{file_id }))))
    }
}
