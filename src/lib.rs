use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use warp::{Filter, Future};

use crate::routes::{get_redirect, health_check, make_shortener};

pub mod configuration;
mod routes;
pub mod surreal;
pub type Db = Surreal<surrealdb::engine::local::Db>;

#[derive(Serialize, Deserialize, Debug)]
struct Redirect {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    id: Thing,
}

pub async fn run(
    config: configuration::Configuration,
) -> surrealdb::Result<(SocketAddr, impl Future<Output = ()>)> {
    let db = surreal::get_db(config.database).await?;
    let shortener = warp::any()
        .and(health_check())
        .or(make_shortener(db.clone()))
        .or(get_redirect(db));

    let server = warp::serve(shortener.with(warp::trace::request()))
        .bind_ephemeral(([0, 0, 0, 0], config.application.port));
    println!("Now Listening on port {}", server.0);
    Ok(server)
}
