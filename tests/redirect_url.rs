use url_shortener::configuration::get_test_configuration;

#[tokio::test]
async fn test_url_redirection() {
    let config = get_test_configuration().expect("Config file is required");
    let (addr, server) = url_shortener::run(config).await.expect("App should run");
    let _ = tokio::spawn(server);
    let client = reqwest::Client::new();
    let url = format!("http://{}/create/https://thekwoka.net", addr);
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Request should complete");
    assert!(response.status().is_success());
    let response_text = response.text().await.expect("Response should have text");
    assert!(response_text.contains("/redirect:"));
    let key = response_text
        .split('/')
        .last()
        .expect("Key should be present");
    let url = format!("http://{}/{}", addr, key);
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Request should complete");
    assert!(response.status().is_success());
    let text_result = response.text().await;
    assert!(text_result.is_ok());

    assert_eq!(text_result.unwrap(), "https://thekwoka.net".to_string());
}
