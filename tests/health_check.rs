use std::net::TcpListener;

// ! test.health_check.rs
use newsletter;
use reqwest;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // let listener bind a random port
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute requests to spawned address.");
    assert!(response.status().is_success());
    // make sure contentlength is of a length > 0
    assert_ne!(Some(0), response.content_length());
}

#[tokio::test]
async fn validsubscribe200() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=austin%20hutchen&email=hutchenaustin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute form fill request");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn invalidform400() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
