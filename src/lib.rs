use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use warp::Filter;

use crate::routes::{get_redirect, make_shortener};

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

pub async fn run() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    let shortener = make_shortener(db.clone()).or(get_redirect(db));

    println!("Listening on port 8080");
    warp::serve(shortener).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}
