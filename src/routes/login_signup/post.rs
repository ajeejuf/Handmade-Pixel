use actix_web::{web, HttpResponse};
use actix_web::http::header::ContentType;
use sqlx::{PgPool, Postgres, Transaction, Executor};
use uuid::Uuid;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use crate::domain::{NewUser, UserEmail, Username};
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub form_type: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl TryFrom<FormData> for NewUser {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let username = Username::parse(value.username)?;
        let email = UserEmail::parse(value.email)?;
        Ok(Self { email, username, password: value.password })
    }
}

#[tracing::instrument(
    name = "Adding a new user",
    skip(form, pool, email_client, base_url),
    fields(
        user_email = %form.email,
        user_name = %form.username,
        user_password = %form.password
    )
)]
pub async fn login_signup(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>
) -> HttpResponse {
    let new_user = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let user_id = match insert_user(&mut transaction, &new_user).await {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let user_token = generate_user_token();
    if store_token(&mut transaction, user_id, &user_token)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    if transaction.commit().await.is_err() {
        return  HttpResponse::InternalServerError().finish();
    }

    if send_confirmation_email(
        &email_client,
        new_user,
        &base_url.0,
        &user_token,
    )
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().content_type(ContentType::html())
        .body(include_str!("login_signup.html"))
}

#[tracing::instrument(
    name = "Store user token in the database",
    skip(user_token, user_id, transaction)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    user_token: &str,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"INSERT INTO user_tokens (user_token, user_id)
        VALUES ($1, $2)"#,
        user_token,
        user_id
    );
    transaction.execute(query)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    Ok(())
}

#[tracing::instrument(
    name = "Sending a confirmation email to a new user",
    skip(email_client, new_user, base_url, user_token)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_user: NewUser,
    base_url: &str,
    user_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/login-signup/confirm?subscription_token={}",
        base_url,
        user_token
    );
    let plain_body = format!(
        "Welcme to Handmade Pixel!\nVisit {} to confirm your account.",
        confirmation_link
    );
    let html_body = format!(
        "Welcome to Handmade Pixel! <br />\
                Click <a href=\"{}\">here</a> to confirm your account.",
        confirmation_link
    );

    email_client
        .send_email(
            new_user.email,
            "Welcome!",
            &html_body,
            &plain_body,
        )
        .await
}

#[tracing::instrument(
    name = "Saving new user details in the database",
    skip(transaction, new_user)
)]
pub async fn insert_user(
    transaction: &mut Transaction<'_, Postgres>,
    new_user: &NewUser,
) -> Result<Uuid, sqlx::Error> {
    let user_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
        INSERT INTO users (id, email, username, password, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')
        "#,
        user_id,
        new_user.email.as_ref(),
        new_user.username.as_ref(),
        new_user.password
    );
    transaction.execute(query)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    Ok(user_id)
}

fn generate_user_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}
