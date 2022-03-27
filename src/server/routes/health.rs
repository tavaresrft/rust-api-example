use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub(super) async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}
