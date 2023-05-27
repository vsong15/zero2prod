use actix_web::HttpResponse;
use actix_web::http::header::LOCATION;

pub async fn login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}
