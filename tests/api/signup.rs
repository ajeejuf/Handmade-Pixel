use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn signup_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "form_type=signup&email=alejandr.fernand%40ufl.edu&username=ajeej&password=password";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_login_signup(body.into()).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn signup_persists_the_new_subscriber() {
    let app = spawn_app().await;
    let body = "form_type=signup&email=alejandr.fernand%40ufl.edu&username=ajeej&password=password";
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_login_signup(body.into()).await;

    let saved = sqlx::query!("SELECT email, username, password, status FROM users",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.email, "alejandr.fernand@ufl.edu");
    assert_eq!(saved.username, "ajeej");
    assert_eq!(saved.password, "password");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "form_type=signup&email=alejandr.fernand%40ufl.edu&username=ajeej&password=password";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_login_signup(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
async fn signup_returns_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("form_type=signup&signup-username=ajeej&signup-password=password", "missing email"),
        ("form_type=signup&email=alejandr.fernand%40gmail.com&signup-password=password", "missing username"),
        ("form_type=signup&email=alejandr.fernand%40gmail.com&signup-username=ajeej", "missing password")
    ];

    for (invalid_body, error_msg) in test_cases {
        let response = app.post_login_signup(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_msg
        );
    }
}

#[tokio::test]
async fn signup_returns_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("form_type=signup&signup-username=ajeej&signup-password=password", "missing email"),
        ("form_type=signup&email=alejandr.fernand%40gmail.com&signup-password=password", "missing username"),
        ("form_type=signup&email=alejandr.fernand%40gmail.com&signup-username=ajeej", "missing password")
    ];

    for (invalid_body, error_msg) in test_cases {
        let response = app.post_login_signup(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_msg
        );
    }
}

