//STD
use std::time::Duration;

//Web framework
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::Timeouts;
use router::Router;

//Database
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

mod file;
mod article;
mod util;

pub type PostgresPool = Pool<PostgresConnectionManager>;


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

fn setup_postgres(conn_str: &'static str, pool_size: u32, min_idle: u32) -> PostgresPool {
    let config = r2d2::Config::builder()
        .pool_size(pool_size)
        .min_idle(Some(min_idle))
        .build();
    let manager = PostgresConnectionManager::new(
        conn_str,
        TlsMode::None).unwrap();
    r2d2::Pool::new(config, manager).unwrap()
}

fn main() {
    let file_db = setup_postgres("postgres://postgres:mysecretpassword@localhost", 16, 4);
    let articles_db = setup_postgres("postgres://postgres:mysecretpassword@localhost", 16, 4);
    let user_db = setup_postgres("postgres://postgres:mysecretpassword@localhost", 16, 4);
    let mut router = Router::new();
    file::register_handlers(file_db, &mut router);
    article::register_handlers(articles_db, &mut router);

    http_listen(router);
}

