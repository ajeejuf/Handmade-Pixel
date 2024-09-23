use actix_web::HttpResponse;
use actix_web::http::header::ContentType;

pub async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../../../static/generated_html/home.html"))
}

pub async fn home1() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("home1.html"))
}