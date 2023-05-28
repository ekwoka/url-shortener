use url_shortener::configuration::get_test_configuration;

#[test]
fn dummy() {
    assert_eq!(0, 0);
}

#[tokio::test]
async fn health_check() {
    let config = get_test_configuration().expect("Config file is required");
    let (addr, server) = url_shortener::run(config).await.expect("App should run");
    tokio::spawn(server);
    let client = reqwest::Client::new();
    let url = format!("http://{}/health_check", addr);
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Request should complete");
    assert!(response.status().is_success());
    match response.text().await {
        Ok(text) => assert_eq!(text, "OK"),
        Err(e) => panic!("Response should have text: {}", e),
    }
}

#[test]
fn validates_url() {
    assert!(url_shortener::routes::ValidURL::parse("https://thekwoka.net".into()).is_ok());
}

#[test]
fn rejects_invalid_url() {
    assert!(url_shortener::routes::ValidURL::parse("thekwoka.net".into()).is_err());
}
