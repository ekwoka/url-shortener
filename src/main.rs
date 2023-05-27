use tracing_subscriber::fmt::format::FmtSpan;
use url_shortener::{configuration, run};

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "tracing=info,warp=info,url_shortener=info".to_owned());

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let config = configuration::get_configuration().expect("config.yaml should be present");
    run(config).await?.1.await;
    Ok(())
}
