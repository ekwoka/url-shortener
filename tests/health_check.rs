use url_shortener::configuration;

#[test]
fn dummy() {
    assert_eq!(0, 0);
}

#[tokio::test]
async fn test_health_check() {
    let mut config = configuration::get_configuration().expect("config.yaml should be present");
    config.application.port = 0;
    let (addr, server) = url_shortener::run(config).await.expect("App should run");
    let _ = tokio::spawn(server);
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
