use iron::response::Response;
use serde::Serialize;
use serde_json;
use modifier::Modifier;
use iron::status;
use iron::headers;
use iron::mime;
use iron::modifier::Set;

pub struct Json<T : Serialize> (pub T);

impl <T> Modifier<Response> for Json<T> where T:Serialize {
    fn modify(self, res: &mut Response) {

        match serde_json::to_vec(&self.0) {
            Err(err) => {
                res.status = Some(status::InternalServerError);
                res.body = Some(Box::new(""));
            }
            Ok(value) => {
                let json_mime: mime::Mime = "application/json".parse().unwrap();
                res.set_mut(json_mime);
                res.body = Some(Box::new(value));
            }
        }
    }
}