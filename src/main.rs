//STD
use std::time::Duration;
use std::option::Option;

//Web framework
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::Timeouts;
use router::Router;

//Database
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

use app::;
use user;

pub type PostgresPool = Pool<PostgresConnectionManager>;
//pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;




fn setup_router(pool: PostgresPool) -> Router {
    let mut router = Router::new();
    let mut a = article::
    router.post("/file", file_create, "file_create");
    router
}


fn http_listen(router: Router) {
    let mut iron = Iron::new(router);
    iron.threads = 8;
    iron.timeouts = Timeouts {
        keep_alive: Some(Duration::from_secs(10)),
        read: Some(Duration::from_secs(10)),
        write: Some(Duration::from_secs(10))
    };
    iron.http("localhost:3000").unwrap();
}

fn setup_postgres(pool_size: u32, min_idle: u32) -> PostgresPool {
    let config = r2d2::Config::builder()
        .pool_size(pool_size)
        .min_idle(Some(min_idle))
        .build();
    let manager = PostgresConnectionManager::new(
        "postgres://postgres@localhost",
        TlsMode::None).unwrap();
    r2d2::Pool::new(config, manager).unwrap()
}

fn main() {
    let pool = setup_postgres(16, 4);
    http_listen(setup_router(pool));
}

