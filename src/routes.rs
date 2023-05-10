use std::convert::Infallible;

use warp::{filters, path::FullPath, Filter};

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
