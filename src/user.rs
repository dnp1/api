pub mod user {
    pub struct Service {
        db: Pool<PostgresConnectionManager>
    }

    impl Service {
        fn user_avatar_update(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_avatar_get(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_email_read(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_email_update_request_create(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_email_update(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_password_update(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_name_read(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");

            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_name_update(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_creation_request_create(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_create(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }

        fn user_password_reset(&self, req: &mut Request) -> IronResult<Response> {
            let ref user_id = get_query_param(req, "user_id");
            Ok(Response::with((status::Ok, *user_id)))
        }


        fn user_session_create(&self, req: &mut Request) -> IronResult<Response> {
            Ok(Response::with(status::Ok))
        }

        pub fn register_on_router(&self, router: Router) {
            router.put("/user/:user_id/avatar", self.user_avatar_update, "user_avatar_update");
            router.get("/user/:user_id/avatar", self.user_avatar_get, "user_avatar_get");
            router.get("/user/:user_id/email", self.user_email_read, "user_email_read");
            router.post("/user/:user_id/email/update", self.user_email_update_request_create, "user_email_update_request_create");
            router.put("/user/:user_id/email", self.user_email_update, "user_email_update");
            router.put("/user/:user_id/password", self.user_password_update, "user_password_update");
            router.get("/user/:user_id/name", self.user_name_read, "user_name_read");
            router.put("/user/:user_id/name", self.user_name_update, "user_name_update");
            router.post("/user/sign-up", self.user_creation_request_create, "user_creation_request_create");
            router.post("/user", self.user_create, "user_create");
            router.post("/user/session", self.session_create, "session_create");
            router.post("/user/password-recovery", self.user_password_reset, "user_password_reset");
        }
    }
}