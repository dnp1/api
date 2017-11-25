use serde::Serialize;
use serde_json;
use iron::status;
use iron::response::Response;
use iron::headers;
use iron::mime;
use iron::modifier::{Modifier, Set};

pub struct Json<T: Serialize>(pub T, pub &'static [u8]);

#[inline]
fn get_json_mime() -> mime::Mime {
    "application/json".parse().unwrap()
}

impl<T> Modifier<Response> for Json<T>
where
    T: Serialize,
{
    #[inline]
    fn modify(self, res: &mut Response) {
        res.set_mut(get_json_mime());
        match serde_json::to_vec(&self.0) {
            Err(_) => {
                res.status = Some(status::InternalServerError);
                res.body = Some(Box::new(self.1));
            }
            Ok(value) => {
                res.body = Some(Box::new(value));
            }
        }
    }
}
