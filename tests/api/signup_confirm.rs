use crate::helpers::spawn_app;
use wiremock::{ResponseTemplate, Mock};
use wiremock::matchers::{path, method};

#[tokio::test]
async fn the_link_returned_by_signup_returns_a_200_if_called() {
    let app = spawn_app().await;
    let body =
        "form_type=signup&email=alejandr.fernand%40ufl.edu&username=ajeej&password=password";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_login_signup(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    let response = reqwest::get(confirmation_links.html)
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;
    let body =
        "form_type=signup&email=alejandr.fernand%40ufl.edu&username=ajeej&password=password";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_login_signup(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let saved = sqlx::query!("SELECT email, username, password, status FROM users",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user.");

    assert_eq!(saved.email, "alejandr.fernand@ufl.edu");
    assert_eq!(saved.username, "ajeej");
    assert_eq!(saved.password, "password");
    assert_eq!(saved.status, "confirmed");
}