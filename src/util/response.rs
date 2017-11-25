use iron::response::Response;
use serde::Serialize;
use serde_json;
use modifier::Modifier;
use iron::status;
use iron::headers;
use iron::mime;
use iron::modifier::Set;

pub struct Json<T : Serialize> (pub T);

#[inline]
fn get_json_mime() -> mime::Mime {
    "application/json".parse().unwrap()
}

impl <T> Modifier<Response> for Json<T> where T:Serialize {
    fn modify(self, res: &mut Response) {
        res.set_mut(get_json_mime());
        match serde_json::to_vec(&self.0) {
            Err(err) => {
                res.status = Some(status::InternalServerError);
                let b = r#"{"code": "E01", "message": "Data serialization has failed"}"#;
                res.body = Some(Box::new(b));
            }
            Ok(value) => {
                res.body = Some(Box::new(value));
            }
        }
    }
}