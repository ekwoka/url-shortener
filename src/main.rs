use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use warp::{http::Response, Filter};

type Db = Surreal<Client>;

#[derive(Serialize, Deserialize, Debug)]
struct Redirect {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    let with_full = warp::any().and(warp::path::full());

    async fn make_url(url: warp::path::FullPath, db: Db) -> Result<String, Infallible> {
        let created: surrealdb::Result<Record> = db
            .create("redirect")
            .content(Redirect {
                url: url.as_str().to_string(),
            })
            .await;
        match created {
            Ok(redirect) => Ok(format!("visit http://localhost:8080/{}", redirect.id)),
            Err(e) => Ok(format!("Error: {}", e)),
        }
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
    let make_shortener = warp::path("create")
        .and(with_full)
        .and(with_db(db.clone()))
        .and_then(make_url);

    let get_shortened = warp::path!(String).and(with_db(db)).and_then(get_url);

    let shortener = make_shortener.or(get_shortened);

    println!("Listening on port 8080");
    warp::serve(shortener).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}
