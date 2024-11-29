use actix_web::HttpResponse;
use actix_web::http::header::ContentType;

pub async  fn learn_more() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("learn_more.html"))
}