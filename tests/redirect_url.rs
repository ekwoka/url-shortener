use url_shortener::configuration::get_test_configuration;

#[tokio::test]
async fn url_redirection() {
    let config = get_test_configuration().expect("Config file is required");
    let (addr, server) = url_shortener::run(config).await.expect("App should run");
    tokio::spawn(server);
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

#[tokio::test]
async fn rejects_invalid_url() {
    let config = get_test_configuration().expect("Config file is required");
    let (addr, server) = url_shortener::run(config).await.expect("App should run");
    tokio::spawn(server);
    let client = reqwest::Client::new();
    let url = format!("http://{}/create/thekwoka.net", addr);
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Request should complete");
    assert_eq!(response.status(), 400);
    let text_result = response.text().await;
    assert!(text_result.is_ok());
    assert_eq!(
        text_result.unwrap(),
        "Error: Invalid URL Target".to_string()
    );
}

#[tokio::test]
async fn invalid_key() {
    let config = get_test_configuration().expect("Config file is required");
    let (addr, server) = url_shortener::run(config).await.expect("App should run");
    tokio::spawn(server);
    let client = reqwest::Client::new();
    let url = format!("http://{}/bad_key", addr);
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Request should complete");
    assert_eq!(response.status(), 400);
    let text_result = response.text().await;
    assert!(text_result.is_ok());
    assert_eq!(text_result.unwrap(), "Error: Invalid ID".to_string());
}

#[tokio::test]
async fn redirect_not_found() {
    let config = get_test_configuration().expect("Config file is required");
    let (addr, server) = url_shortener::run(config).await.expect("App should run");
    tokio::spawn(server);
    let client = reqwest::Client::new();
    let url = format!("http://{}/redirect:bad_key", addr);
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Request should complete");
    assert_eq!(response.status(), 404);
    let text_result = response.text().await;
    assert!(text_result.is_ok());
    assert_eq!(text_result.unwrap(), "Not Found".to_string());
}
