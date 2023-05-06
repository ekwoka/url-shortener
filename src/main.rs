use std::{collections::HashMap, convert::Infallible, sync::Arc};

use tokio::sync::Mutex;
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    pub type Db = Arc<Mutex<HashMap<i32, String>>>;
    let db: Db = Mutex::new(HashMap::new()).into();

    #[derive(Clone)]
    struct NextKey {
        next_id: i32,
    }
    let next_id = Arc::new(Mutex::new(NextKey { next_id: 0 }));

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn with_key(
        next_id: Arc<Mutex<NextKey>>,
    ) -> impl Filter<Extract = (Arc<Mutex<NextKey>>,), Error = std::convert::Infallible> + Clone
    {
        warp::any().map(move || next_id.clone())
    }

    let with_full = warp::any().and(warp::path::full());

    async fn make_url(
        url: warp::path::FullPath,
        db: Db,
        key: Arc<Mutex<NextKey>>,
    ) -> Result<String, Infallible> {
        let mut count = key.lock().await;
        count.next_id += 1;
        db.lock().await.insert(
            count.next_id,
            url.as_str().to_string().replace("/create/", ""),
        );
        Ok(format!("visit http://localhost:8080/{}", count.next_id))
    }

    async fn get_url(id: String, db: Db) -> Result<Response<String>, Infallible> {
        let db = db.lock().await;
        let Ok(id) = &id.parse::<i32>() else {
            return Ok(Response::builder()
                .status(200)
                .body("Invalid URL - Needs to be Number".to_string()).unwrap());
        };
        match db.get(id) {
            Some(url) => Ok(Response::builder()
                .status(308)
                .header("Location", url)
                .body("".to_string())
                .unwrap()),
            None => Ok(Response::builder()
                .status(200)
                .body("Invalid URL - Not Registered Yet".to_string())
                .unwrap()),
        }
    }
    let make_shortener = warp::path("create")
        .and(with_full)
        .and(with_db(db.clone()))
        .and(with_key(next_id.clone()))
        .and_then(make_url);

    let get_shortened = warp::path!(String).and(with_db(db)).and_then(get_url);

    let shortener = make_shortener.or(get_shortened);

    println!("Listening on port 8080");
    warp::serve(shortener).run(([0, 0, 0, 0], 8080)).await;
}
