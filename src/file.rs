pub mod file {
    struct Service {
        db: Pool<PostgresConnectionManager>
    }

    fn file_create(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with(status::Ok))
    }
}