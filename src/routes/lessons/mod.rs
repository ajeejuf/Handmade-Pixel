use actix_web::HttpResponse;
use actix_web::http::header::ContentType;

pub async  fn lessons() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("lessons.html"))
}