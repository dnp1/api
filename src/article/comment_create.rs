use iron::Response;
use iron::IronResult;
use iron::status;
use std::error::Error;

use iron_simple::SimpleHandler;
use util::json;
use uuid::Uuid;

use super::{AuthenticatedSession, Services};


pub struct Handler;

#[derive(RequestRouteParams)]
pub struct RouteParams {
    article_id: Uuid
}

#[derive(Clone, Deserialize, RequestBody)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    content: String,
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    comment_id: Uuid
}

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (RouteParams, AuthenticatedSession, Body);

    fn handle(&self, req: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let (route_params, session, body) = req;

        let comment_id: Uuid = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(conn) => match conn.query(
                "SELECT create_comment($1, $2, $3) as comment_id",
                &[&route_params.article_id, &session.user_id, &body.content]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => {
                    if rows.len() > 0 {
                        rows.get(0).get("comment_id")
                    } else {
                        return Ok(Response::with((status::InternalServerError, "")))
                    }
                }
            },
        };

        Ok(Response::with((status::Ok, json(Comment{comment_id}))))
    }
}