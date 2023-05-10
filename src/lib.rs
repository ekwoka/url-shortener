use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use warp::{http::Response, Filter};

use crate::routes::make_shortener;

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

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
    async fn get_url(id: String, _db: Db) -> Result<Response<String>, Infallible> {
        let Ok(_id) = &id.parse::<i32>() else {
            return Ok(Response::builder()
                .status(200)
                .body("Invalid URL - Needs to be Number".to_string()).unwrap());
        };
        Ok(Response::builder()
            .status(200)
            .body("Invalid URL - Not Registered Yet".to_string())
            .unwrap())
    }

    let get_shortened = warp::path!(String)
        .and(with_db(db.clone()))
        .and_then(get_url);

    let shortener = make_shortener(db.clone()).or(get_shortened);

    println!("Listening on port 8080");
    warp::serve(shortener).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}
