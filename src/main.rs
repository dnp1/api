//JSON
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate chrono;

//STD
use std::time::Duration;
use std::sync::Arc;
use std::collections::HashSet;


//JWT
extern crate jsonwebtoken as jwt;

//uuid
extern crate uuid;

//Web framework
extern crate iron;
extern crate router;
extern crate params;
extern crate bodyparser;
extern crate iron_cors;


use iron::prelude::*;
use iron::Timeouts;
use iron::Chain;
use iron::Handler;
use router::Router;
use iron_cors::CorsMiddleware;

//Database
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;

mod file;
mod article;
mod user;
mod util;

pub type PostgresPool = Pool<PostgresConnectionManager>;


fn http_listen<T> (h: T) where T: Handler {
    let mut iron = Iron::new(h);
    iron.threads = 8;
    iron.timeouts = Timeouts {
        keep_alive: Some(Duration::from_secs(10)),
        read: Some(Duration::from_secs(10)),
        write: Some(Duration::from_secs(10))
    };
    iron.http("0.0.0.0:3000").unwrap();
}

fn setup_postgres(conn_str: &str, pool_size: u32, min_idle: u32) -> PostgresPool {
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
    let sm = util::SessionManager::new("sadnash dsa das");
    let file_db = setup_postgres("postgres://postgres:mysecretpassword@localhost/file", 10, 10);
    let articles_db = setup_postgres("postgres://postgres:mysecretpassword@localhost/article", 10, 10);
    let user_db = setup_postgres("postgres://postgres:mysecretpassword@localhost/user", 10, 10);
    let mut router = Router::new();
    let sm = Arc::from(sm);
    let storage = util::DiskStorage::new("/home/danilo/uploads");
    file::register_handlers(file_db, &mut router, sm.clone(), Arc::from(storage));
    article::register_handlers(articles_db, &mut router, sm.clone());
    user::register_handlers(user_db, &mut router, sm.clone());
    let allowed_hosts = ["http://localhost:8080"].iter().map(ToString::to_string).collect::<HashSet<_>>();
    let mut cors_middleware = CorsMiddleware::with_whitelist(allowed_hosts);
    cors_middleware.allow_credentials();

    let mut chain = Chain::new(router);
    chain.link_around(cors_middleware);

    http_listen(chain);
}

