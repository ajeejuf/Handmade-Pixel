use actix_web::HttpResponse;
use actix_web::http::header::ContentType;

pub async  fn login_signup_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login_signup.html"))
}