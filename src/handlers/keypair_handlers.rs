use actix_web::{HttpResponse, Responder, post};

use crate::{
    models::{ErrorResponse, SuccessResponse},
    services::keypair_services::create_keyair,
};

#[post("/keypair")]
pub async fn generate_keypair() -> impl Responder {
    match create_keyair() {
        Ok(data) => HttpResponse::Ok().json(SuccessResponse {
            success: true,
            data,
        }),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: err,
        }),
    }
}
