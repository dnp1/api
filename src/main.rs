//JSON
#[macro_use]
extern crate src_derive;

#[macro_use]
extern crate iron_simple_derive;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate chrono;

extern crate iron_simple;

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
extern crate bodyparser;
extern crate iron_cors;
extern crate iron_json_response_modifier;


use iron::prelude::*;
use iron::Timeouts;
use iron::Chain;
use iron::Handler;
use router::Router;
use router::Params;
use iron_cors::CorsMiddlewareBuilder;

//Database
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate postgres_inet;
extern crate futures_cpupool;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::Pool;
use futures_cpupool::CpuPool;

mod file;
mod article;
mod user;
mod util;

pub type PostgresPool = Pool<PostgresConnectionManager>;


fn http_listen<T>(h: T) where T: Handler {
    let mut iron = Iron::new(h);
    iron.pool = CpuPool::new(8);
    iron.timeouts = Timeouts {
        keep_alive: Some(Duration::from_secs(1)),
        read: Some(Duration::from_secs(5)),
        write: Some(Duration::from_secs(5))
    };
    iron.http("0.0.0.0:3000");
}

fn setup_postgres(conn_str: &str, pool_size: u32, min_idle: u32) -> PostgresPool {
    let manager = PostgresConnectionManager::new(
        conn_str,
        TlsMode::None).unwrap();
    r2d2::Pool::builder()
        .max_size(pool_size)
        .min_idle(Some(min_idle))
        .build(manager).unwrap()
}

fn main() {
    let session_manager = util::session_manager::SessionManager::new("sadnash dsa das");
    let file_db = setup_postgres("postgres://postgres:mysecretpassword@localhost/file", 30, 10);
    let articles_db = setup_postgres("postgres://postgres:mysecretpassword@localhost/article", 30, 10);
    let user_db = setup_postgres("postgres://postgres:mysecretpassword@localhost/user", 30, 10);
    let mut router = Router::new();
    let storage = util::DiskStorage::new("/home/danilo/uploads");
    file::register_handlers(file_db, &mut router, session_manager.clone(), storage);
    article::register_handlers(articles_db, &mut router, session_manager.clone());

    user::register_handlers(user_db, &mut router, session_manager.clone());

    let allowed_hosts = ["http://localhost:8080", "http://danilo.local:8080"].iter().map(ToString::to_string).collect::<HashSet<_>>();
    let cors_middleware = CorsMiddlewareBuilder::new()
        .allowed_hosts(allowed_hosts)
        .allow_credentials(true)
        .build();

    let mut chain = Chain::new(router);
    chain.link_around(cors_middleware);

    http_listen(chain);
}

