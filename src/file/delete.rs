use std::sync::Arc;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use iron::prelude::Response;
use iron::prelude::Request;
use iron::prelude::IronResult;
use iron::status;
use std::error::Error;
use uuid::Uuid;
use util::{Storage, Session, SimpleHandler, Empty, SimpleRequest, FromRouteParams, json};
use std::str::FromStr;

pub struct Handler {
    pub db: Arc<Pool<PostgresConnectionManager>>,
}

#[derive(FromRouteParams)]
pub struct RouteParams {
    file_id: Uuid
}

impl SimpleHandler<RouteParams, Empty, Empty, Empty> for Handler {
    fn authenticated(&self) -> bool {
        true
    }
    fn handle(&self, req: &SimpleRequest<RouteParams, Empty, Empty, Empty>, session: &mut Session) -> IronResult<Response> {

        let ok: Option<bool> = match self.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query("SELECT deactivate_file($1, $2)",
                                         &[&req.route_params.file_id, &session.user_id]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => if rows.len() > 0 {
                    rows.get(0).get(0)
                } else {
                    return Ok(Response::with((status::NotFound, "file was not found")))
                }
            }
        };
        match ok {
            None => Ok(Response::with((status::NotFound, "file was not found"))),
            Some(false) =>  Ok(Response::with((status::Forbidden, "You don't have authorization for delete this file"))),
            Some(true) =>  Ok(Response::with((status::Ok, "")))
        }
    }
}