use iron_simple::SimpleErrorTransformer;
use iron_simple::SimpleError;
use iron::{IronResult, Response, Set};
use iron::modifier::Modifier;
use util::json;

#[derive(Clone, Serialize)]
struct JsonError {
    message: String
}

impl Modifier<Response> for JsonError {
    #[inline]
    fn modify(self, res: &mut Response) {
        res.set_mut(json(self));
    }
}

pub struct JsonErrorTransformer;

impl SimpleErrorTransformer for JsonErrorTransformer {
    fn transform(&self, err: SimpleError) -> IronResult<Response> {
        return Ok(Response::with((err.status(), JsonError{message: err.description().to_owned()})))
    }
}

