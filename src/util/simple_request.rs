use std::sync::Arc;
use bodyparser;
use serde::de::DeserializeOwned;
use uuid::Uuid;
use std::str::FromStr;
use iron::Request;
use iron::Plugin;
use iron::Handler;
use iron::IronResult;
use iron::Response;
use iron::status;
use util::set_cookie;
use util::session::SessionManager;
use util::Session;

pub trait FromRequest {
    fn from_request<'a>(req: &'a mut Request) -> Self;
}


pub trait FromRouteParams<T> {
    fn from_route_params<'a>(params: &::router::Params) -> Result<T, ()>;
}

pub struct SimpleRequest<R, Q, B, S>
{
    pub route_params: R,
    pub query_params: Q,
    pub body: B,
    pub extra: S,
}

impl<R, Q, B, S> SimpleRequest<R, Q, B, S>
    where R: FromRouteParams<R> + 'static,
          Q: 'static + DeserializeOwned + Clone,
          B: 'static + DeserializeOwned + Clone,
          S: 'static + DeserializeOwned + Clone + FromRequest
{
    fn from_request<'a>(req: &'a mut Request) -> Result<Self, &'a str> {
        let route = match R::from_route_params(req.extensions.get::<::router::Router>().unwrap()) {
            Err(_) => return Err("dsadas"),
            Ok(val) => val,
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
            route_params: route,
            query_params: query,
            body,
            extra: S::from_request(req),
        })
    }
}


#[derive(Clone, Deserialize)]
pub struct Empty;

impl FromRequest for Empty {
    fn from_request<'a>(req: &'a mut Request) -> Self {
        Empty
    }
}

pub trait SimpleHandler<R, Q, B, S>
    where R: FromRouteParams<R>,
          Q: DeserializeOwned + Clone,
          B: DeserializeOwned + Clone,
          S: DeserializeOwned + Clone
{
    fn authenticated(&self) -> bool {
        false
    }
    fn handle(&self, req: &SimpleRequest<R, Q, B, S>, session: &mut Session) -> IronResult<Response>;
}

pub struct SimpleHandlerBox<T, R, Q, B>
    where T: SimpleHandler<R, Q, B, Empty> + Send + Sync + 'static,
          R: FromRouteParams<R>,
          Q: DeserializeOwned + Clone,
          B: DeserializeOwned + Clone,
{
    pub handler: T,
    pub sm: Arc<SessionManager>,
    r: ::std::marker::PhantomData<R>,
    q: ::std::marker::PhantomData<Q>,
    b: ::std::marker::PhantomData<B>,
}

impl <T, R, Q, B> SimpleHandlerBox<T, R, Q, B>
    where T: SimpleHandler<R, Q, B, Empty> + Send + Sync + 'static,
          R: FromRouteParams<R>,
          Q: DeserializeOwned + Clone,
          B: DeserializeOwned + Clone,
{
    pub fn new(handler: T, sm: Arc<SessionManager>) -> Self {
        SimpleHandlerBox {
            handler,
            sm,
            r: ::std::marker::PhantomData,
            q: ::std::marker::PhantomData,
            b: ::std::marker::PhantomData,
        }
    }
}


impl<T, R, Q, B> Handler for SimpleHandlerBox<T, R, Q, B>
    where T: SimpleHandler<R, Q, B, Empty> + Send + Sync + 'static,
          R: FromRouteParams<R> + Send + Sync + 'static,
          Q: 'static + DeserializeOwned + Clone + Send + Sync + 'static,
          B: 'static + DeserializeOwned + Clone + Send + Sync + 'static
{
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut session = match self.sm.get_request_session(req) {
            None => return Ok(Response::with((status::Unauthorized, "You must create a session"))),
            Some(session) => {
                if self.handler.authenticated() {
                    if let None = session.user_id {
                        return Ok(Response::with((status::Unauthorized, "You must authenticate with an user")));
                    }
                }
                session
            }
        };

        let mut r: SimpleRequest<R, Q, B, Empty> = match SimpleRequest::from_request(req) {
            Err(s) => return Ok(Response::with((status::BadRequest, "You must create a session"))),
            Ok(val) => val,
        };

        match self.handler.handle(&r, &mut session) {
            Ok(mut response) => {
                match self.sm.create_session_payload(&mut session) {
                    Err(err) => Ok(Response::with((status::InternalServerError, err.to_string()))),
                    Ok(payload) => {
                        set_cookie(&mut response, &payload);
                        Ok(response)
                    }
                }
            }
            Err(err) => Err(err),
        }
    }
}