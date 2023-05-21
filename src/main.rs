use url_shortener::{configuration, run};

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    let config = configuration::get_configuration().expect("config.yaml should be present");
    run(config).await
}
