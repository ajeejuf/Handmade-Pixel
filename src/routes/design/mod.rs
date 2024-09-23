use actix_web::HttpResponse;
use actix_web::http::header::ContentType;

pub async  fn design() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("design.html"))
}