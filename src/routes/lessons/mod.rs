use actix_web::HttpResponse;
use actix_web::http::header::ContentType;

pub async  fn lessons() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../../../static/generated_html/lessons/lessons.html"))
}

pub async  fn lesson1() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../../../static/generated_html/lesson1/lesson1.html"))
}

pub async  fn lesson2() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("../../../static/generated_html/lesson2/lesson2.html"))
}

pub async  fn lesson3() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("lesson3.html"))
}

pub async  fn lesson4() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("lesson4.html"))
}

pub async  fn lesson5() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("lesson5.html"))
}

pub async  fn lesson6() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("lesson6.html"))
}