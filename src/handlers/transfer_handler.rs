use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    models::{ErrorResponse, SendSolRequest, SendTokenRequest, SuccessResponse},
    services::transfer_service::{create_send_sol_ix, create_send_token_ix},
};

#[post("/send/sol")]
pub async fn send_sol(body: web::Json<SendSolRequest>) -> impl Responder {
    match create_send_sol_ix(body) {
        Ok(data) => HttpResponse::Ok().json(SuccessResponse {
            success: true,
            data,
        }),
        Err(err) => HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: err,
        }),
    }
}

#[post("/send/token")]
pub async fn send_token(body: web::Json<SendTokenRequest>) -> impl Responder {
    match create_send_token_ix(body) {
        Ok(data) => HttpResponse::Ok().json(SuccessResponse {
            success: true,
            data,
        }),
        Err(err) => HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: err,
        }),
    }
}
