use iron::status;
use iron::Response;
use iron::IronResult;

use uuid::Uuid;
use iron_simple::SimpleHandler;

use article::common::Article;
use util::json;
use super::{Session, Services};
use std::error::Error;

const FETCH_LENGTH: i32 = 10;

pub struct Handler;

impl SimpleHandler for Handler {
    type Services = Services;
    type Request = (Session,);

    fn handle(&self, _: Self::Request, services: &Self::Services) -> IronResult<Response> {
        let after_uuid: Option<Uuid> = None; //TODO:get_query_param
        let resp: Vec<Article> = match services.db.get() {
            Err(err) => return Ok(Response::with((status::ServiceUnavailable, err.description()))),
            Ok(connection) => match connection.query("SELECT * FROM get_article_list($1, $2)",
                                                     &[&FETCH_LENGTH, &after_uuid]) {
                Err(err) => return Ok(Response::with((status::InternalServerError, err.description()))),
                Ok(rows) => (&rows).iter().map(|row| Article::from_row(&row)).collect()
            }
        };
        Ok(Response::with((status::Ok, json(resp))))
    }
}