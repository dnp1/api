use iron::status;
use iron::Response;
use iron::IronResult;

use uuid::Uuid;
use iron_simple::SimpleHandler;

use util::json;
use super::{AuthenticatedSession, Services};
use std::error::Error;
use util::storage::Storage;

#[derive(RequestRouteParams)]
pub struct RouteParams {
    file_id: Uuid
}

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, AuthenticatedSession);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, session) = req;

        let ok: Option<bool> = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query("SELECT deactivate_file($1, $2)",
                                         &[&route_params.file_id, &session.user_id]) {
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