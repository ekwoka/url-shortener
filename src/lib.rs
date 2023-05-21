use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use warp::{Filter, Future};

use crate::routes::{get_redirect, health_check, make_shortener};

pub mod configuration;
mod routes;
pub type Db = Surreal<Client>;

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
    let db = Surreal::new::<Ws>(config.database.connection_string()).await?;

    db.signin(Root {
        username: &config.database.username,
        password: &config.database.password,
    })
    .await?;

    db.use_ns("test")
        .use_db(&config.database.database_name)
        .await?;

    let shortener = health_check()
        .or(make_shortener(db.clone()))
        .or(get_redirect(db));

    println!("Now Listening on port {}", &config.application.port);
    let server = warp::serve(shortener).bind_ephemeral(([0, 0, 0, 0], config.application.port));
    Ok(server)
}
