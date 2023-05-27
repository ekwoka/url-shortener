use std::convert::Infallible;

use surrealdb::sql::Thing;
use warp::{filters, http::Response, path::FullPath, Filter};

use crate::{Record, Redirect};

struct Id(Thing);

impl TryFrom<String> for Id {
    type Error = String;
    fn try_from(id: String) -> Result<Self, Self::Error> {
        match id.split_once(':') {
            Some((tb, id)) => Ok(Self(Thing::from((tb.to_string(), id.to_string())))),
            None => Err("Invalid ID".into()),
        }
    }
}

impl From<Id> for Thing {
    fn from(val: Id) -> Self {
        val.0
    }
}

fn with_full() -> filters::BoxedFilter<(FullPath,)> {
    warp::any().and(warp::path::full()).boxed()
}

pub fn make_shortener(db: crate::Db) -> filters::BoxedFilter<(String,)> {
    async fn make_url(url: warp::path::FullPath, db: crate::Db) -> Result<String, Infallible> {
        let destination = url.as_str().replace("/create/", "");
        tracing::info!("creating redirect to {}", destination);
        let created: surrealdb::Result<Record> = db
            .create("redirect")
            .content(Redirect {
                url: destination.to_string(),
            })
            .await;
        match created {
            Ok(redirect) => Ok(format!("visit http://localhost:8080/{}", redirect.id)),
            Err(e) => Ok(format!("Error: {}", e)),
        }
    }

    warp::path("create")
        .and(with_full())
        .and(warp::any().map(move || db.clone()))
        .and_then(make_url)
        .boxed()
}

pub fn get_redirect(db: crate::Db) -> filters::BoxedFilter<(Response<String>,)> {
    fn with_db(db: crate::Db) -> filters::BoxedFilter<(crate::Db,)> {
        warp::any().map(move || db.clone()).boxed()
    }
    async fn get_url(id: String, db: crate::Db) -> Result<Response<String>, Infallible> {
        let id: Id = match Id::try_from(id) {
            Ok(id) => id,
            Err(e) => {
                return Ok(Response::builder()
                    .status(400)
                    .body(format!("Error: {}", e))
                    .unwrap())
            }
        };
        let redirect: Result<Redirect, surrealdb::Error> = db.select(Into::<Thing>::into(id)).await;

        match redirect {
            Ok(redirect) => Ok(Response::builder().status(200).body(redirect.url).unwrap()),
            Err(surrealdb::Error::Api(surrealdb::error::Api::FromValue { value: _, error: _ })) => {
                Ok(Response::builder()
                    .status(404)
                    .body("Not Found".to_string())
                    .unwrap())
            }
            Err(e) => Ok(Response::builder()
                .status(400)
                .body(format!("Error: {}", e))
                .unwrap()),
        }
    }

    warp::path!(String)
        .and(with_db(db))
        .and_then(get_url)
        .boxed()
}

pub fn health_check() -> filters::BoxedFilter<(Response<String>,)> {
    warp::path!("health_check")
        .and(warp::get())
        .map(|| {
            Response::builder()
                .status(200)
                .body("OK".to_string())
                .unwrap()
        })
        .boxed()
}
