use edgedb_tokio::Error;
use email_sub::{db_connection::connection, startup::run};
use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind to a random port.");
    let connection = connection().await.expect("Failed to connect to database");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, connection).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app().await;
    let _db = connection().await.expect("Failed to connect to database");
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    let app_address = spawn_app().await;
    let db = connection().await.expect("Failed to connect to database");
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let greeting = db
        .query_required_single::<String, _>("SELECT subscriptions", &())
        .await
        .expect("Failed to execute query");
    assert_eq!("hello", greeting);
}

#[tokio::test]
async fn subscribe_return_a_400_when_data_is_missing() {
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
