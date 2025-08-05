use actix_web::{HttpResponse, Responder, post, web};

use crate::{
    models::{CreateTokenRequest, ErrorResponse, MintTokenRequest, SuccessResponse},
    services::token_services::{create_initialize_mint_ix, create_mint_token_ix},
};

#[post("/token/create")]
pub async fn create_token(body: web::Json<CreateTokenRequest>) -> impl Responder {
    match create_initialize_mint_ix(body) {
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

#[post("/token/mint")]
pub async fn mint_token(body: web::Json<MintTokenRequest>) -> impl Responder {
    match create_mint_token_ix(body) {
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
