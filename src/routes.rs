use std::convert::Infallible;

use warp::{filters, http::Response, path::FullPath, Filter};

use crate::{Record, Redirect};

fn with_full() -> filters::BoxedFilter<(FullPath,)> {
    warp::any().and(warp::path::full()).boxed()
}

pub fn make_shortener(db: crate::Db) -> filters::BoxedFilter<(String,)> {
    async fn make_url(url: warp::path::FullPath, db: crate::Db) -> Result<String, Infallible> {
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

    warp::path!("create")
        .and(with_full())
        .and(warp::any().map(move || db.clone()))
        .and_then(make_url)
        .boxed()
}

pub fn get_redirect(db: crate::Db) -> filters::BoxedFilter<(Response<String>,)> {
    fn with_db(db: crate::Db) -> filters::BoxedFilter<(crate::Db,)> {
        warp::any().map(move || db.clone()).boxed()
    }
    async fn get_url(id: String, _db: crate::Db) -> Result<Response<String>, Infallible> {
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

    warp::path!(String)
        .and(with_db(db))
        .and_then(get_url)
        .boxed()
}
