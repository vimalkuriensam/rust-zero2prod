// // ! tests/health_check.rs

use tokio::spawn;
use zero2prod::run;

#[tokio::test]
#[allow(unused_must_use)]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let server = run();
    let _ = spawn(server);
}
