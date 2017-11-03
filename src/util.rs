pub mod util {
    fn get_query_param<'s>(req: &'s Request, name: &'s str) -> &'s str {
        return req.extensions.get::<Router>().unwrap().find(name).unwrap_or("/");
    }
}