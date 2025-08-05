use actix_web::{App, HttpResponse, HttpServer, Responder, get};

use crate::handlers::{
    create_token, generate_keypair, mint_token, send_sol, send_token, sign_message, verify_message,
};
mod handlers;
mod models;
mod services;

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("server is healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(generate_keypair)
            .service(create_token)
            .service(mint_token)
            .service(sign_message)
            .service(verify_message)
            .service(send_sol)
            .service(send_token)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
