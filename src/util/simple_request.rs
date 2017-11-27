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
use std::error::Error;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum ClientError {
    MissingRouteParam(String),
    Other
}


impl ::std::error::Error for ClientError {
    fn description(&self) -> &str {
        match *self {
            ClientError::MissingRouteParam(ref msg) => msg,
            _ => "Could not save file"
        }

    }
}

impl ::std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "TODO: Oh no, something bad went down")
    }
}

pub trait FromRequest<T> : Send + Sync + 'static {
    fn from_request<'a>(req: &'a Request) -> IronResult<T> ;
}

pub trait FromRouteParams<T>: Send + Sync + 'static {
    fn from_params<'a>(params: &::router::Params) -> IronResult<T>;
    fn from_request<'a>(req: &'a Request) -> IronResult<T> {
        Self::from_params(req.extensions.get::<::router::Router>().unwrap())
    }
}

pub trait FromUrlEncoded<T>: Send + Sync + 'static {
    fn from_request<'a>(req: &::iron::Request) -> IronResult<T>;
}

pub trait FromBodyParser<T>: Send + Sync + 'static {
    fn from_request<'a>(req: &::iron::Request) -> IronResult<T>;
}

pub trait FromQueryParams<T>: Send + Sync + 'static {
    fn from_request<'a>(req: &::iron::Request) -> IronResult<T>;
}

pub struct SimpleRequest<R, Q, B, S>
{
    pub route_params: R,
    pub query_params: Q,
    pub body: B,
    pub extra: S,
}

impl<R, Q, B, S> SimpleRequest<R, Q, B, S>
    where R: FromRouteParams<R>,
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
          S: FromRequest<S>,
{
    fn from_request<'a>(req: &'a Request) -> IronResult<Self> {
        let route_params = match R::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };
        let query_params = match Q::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };
        let body = match B::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };
        let extra = match S::from_request(req) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };


        Ok(SimpleRequest {
            route_params,
            query_params,
            body,
            extra,
        })
    }
}


#[derive(Clone, Deserialize)]
pub struct Empty;

impl FromRequest<Empty> for Empty {
    fn from_request<'a>(_: &'a Request) -> IronResult<Empty> {
        Ok(Empty)
    }
}


impl FromRouteParams<Empty> for Empty  {
    fn from_params<'a>(_: &::router::Params) -> IronResult<Empty> {
        Ok(Empty)
    }
}

impl FromUrlEncoded<Empty> for Empty {
    fn from_request<'a>(_: &'a Request) -> IronResult<Empty> {
        Ok(Empty)
    }
}

impl FromBodyParser<Empty> for Empty {
    fn from_request<'a>(_: &'a Request) -> IronResult<Empty> {
        Ok(Empty)
    }
}

impl FromQueryParams<Empty> for Empty {
    fn from_request<'a>(_: &'a Request) -> IronResult<Empty> {
        Ok(Empty)
    }
}


pub trait SimpleHandler<R, Q, B, S>
    where R: FromRouteParams<R>,
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
          S: FromRequest<S>,
{
    fn authenticated(&self) -> bool {
        false
    }
    fn handle(&self, req: &SimpleRequest<R, Q, B, S>, session: &mut Session) -> IronResult<Response>;
}

pub struct SimpleHandlerBox<T, R, Q, B>
    where T: SimpleHandler<R, Q, B, Empty> + Send + Sync + 'static,
          R: FromRouteParams<R>,
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
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
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
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
          R: FromRouteParams<R>,
          Q: FromRequest<Q>,
          B: FromBodyParser<B>,
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

        let r: SimpleRequest<R, Q, B, Empty> = match SimpleRequest::from_request(req) {
            Err(s) => {
                println!("{}", s);
                ::std::io::stdout().flush().unwrap();
                return Ok(Response::with((status::BadRequest, "Could not parse body")))
            },
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