use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    models::{ErrorResponse, SignMessageRequest, SuccessResponse, VerfiySignatureRequest},
    services::message_services::{sign_message_ix, verify_message_ix},
};

#[post("/message/sign")]
pub async fn sign_message(body: web::Json<SignMessageRequest>) -> impl Responder {
    match sign_message_ix(body) {
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

#[post("/message/verify")]
pub async fn verify_message(body: web::Json<VerfiySignatureRequest>) -> impl Responder {
    match verify_message_ix(body) {
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
