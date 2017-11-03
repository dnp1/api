pub mod article {
    pub struct Service<'s> {
        db: Pool<PostgresConnectionManager>
    }

    impl Service {
        fn article_list(&self, req: &mut Request) -> IronResult<Response> {
            //NO Query;
            Ok(Response::with((status::Ok, "")))
        }

        fn article_read(&self, req: &mut Request) -> IronResult<Response> {
            let ref article_id = get_query_param(req, "article_id");
            Ok(Response::with((status::Ok, *article_id)))
        }

        fn article_archive(&self, req: &mut Request) -> IronResult<Response> {
            Ok(Response::with((status::Ok, "")))
        }

        fn article_tag_list(&self, req: &mut Request) -> IronResult<Response> {
            let ref article_id = get_query_param(req, "article_id");
            Ok(Response::with((status::Ok, *article_id)))
        }

        fn article_comment_list(&self, req: &mut Request) -> IronResult<Response> {
            let ref article_id = get_query_param(req, "article_id");
            Ok(Response::with((status::Ok, *article_id)))
        }

        fn article_comment_read(&self, req: &mut Request) -> IronResult<Response> {
            let ref article_id = get_query_param(req, "article_id");
            Ok(Response::with((status::Ok, *article_id)))
        }

        fn article_comment_create(&self, req: &mut Request) -> IronResult<Response> {
            let ref article_id = get_query_param(req, "article_id");
            Ok(Response::with((status::Ok, *article_id)))
        }

        pub fn register_on_router(&self, router: Router) {
            router.get("/article", self.article_list, "article_list");
            router.get("/article/:article_id", self.article_read, "article_read");
            router.get("/article/archive", self.article_archive, "article_archive");
            router.get("/article/:article_id/tag", self.article_tag_list, "article_tag_list");
            router.get("/article/:article_id/comment", self.article_comment_list, "article_comment_list");
            router.get("/article/:article_id/comment/:comment_id", self.article_comment_read, "article_comment_read");
            router.post("/article/:article_id/comment", self.article_comment_create, "article_comment_create");
        }
    }
}