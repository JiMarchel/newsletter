pub mod configuration;
pub mod routes;
pub mod starup;

use actix_web::HttpResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
