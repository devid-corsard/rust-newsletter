use actix_web::HttpResponse;
use reqwest::header::LOCATION;

pub fn e500<E>(e: E) -> actix_web::Error
where
    E: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn see_other(path: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish()
}
