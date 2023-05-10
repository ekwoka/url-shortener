use url_shortener::run;

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    run().await
}
