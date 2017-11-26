use bodyparser;
use serde::de::DeserializeOwned;
use uuid::Uuid;
use std::str::FromStr;
use iron::Request;
use iron::Plugin;

pub trait FromRouteParams<T> {
    fn from_route_params<'a>(params : &::router::Params) -> Result<T, ()>;
}


#[derive(FromRouteParams)]
pub struct RP {
    a: String
}

pub trait FromRequest {
    fn from_request<'a>(req: &'a mut Request) -> Self;
}

pub struct SimpleRequest<R, Q, B, E>
    where R: DeserializeOwned + Clone, Q: DeserializeOwned + Clone, B: DeserializeOwned + Clone, E: FromRequest
{
    route: R,
    query : Q,
    body: B,
    extra: E
}

impl <'a, R, Q, B, E> SimpleRequest<R, Q, B, E>
    where R: 'static + DeserializeOwned + Clone, Q:  'static + DeserializeOwned + Clone, B: 'static + DeserializeOwned + Clone, E: FromRequest
{
    fn from_request(req: &'a mut Request) -> Result<Self, &'a str> {
        let route = match req.get::<bodyparser::Struct<R>>() {
            Err(err) => return Err("dsadas"),
            Ok(None) => return Err("empty body"),
            Ok(Some(struct_body)) => struct_body,
        };
        let query = match req.get::<bodyparser::Struct<Q>>() {
            Err(err) => return Err("dsdasda"),
            Ok(None) => return Err("empty body"),
            Ok(Some(struct_body)) => struct_body,
        };
        let body = match req.get::<bodyparser::Struct<B>>() {
            Err(err) => return Err("dsadasd"),
            Ok(None) => return Err("empty body"),
            Ok(Some(struct_body)) => struct_body,
        };

        Ok(SimpleRequest {
            route: route.clone(),
            query: query.clone(),
            body: body.clone(),
            extra: E::from_request(req),
        })
    }
}