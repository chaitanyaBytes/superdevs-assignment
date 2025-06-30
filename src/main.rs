use axum::{
    Json, Router,
    response::{self, IntoResponse},
    routing::{get, post},
};
use base64::{Engine as _, engine::general_purpose};
use bs58;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    hash::hash,
    message::Message,
    signature::{Keypair, Signer},
    system_program,
};
use spl_token::{
    ID as TOKEN_PROGRAM_ID,
    instruction::{initialize_mint, mint_to},
};
#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/keypair", post(keygen))
        .route("/token/create", post(create_token));
    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

// keypair response

#[derive(Serialize)]
struct KeypairResponse {
    success: bool,
    data: KeypairData,
}

#[derive(Serialize)]
struct KeypairData {
    pubkey: String,
    secret: String,
}

async fn keygen() -> impl IntoResponse {
    let keypair = Keypair::new();

    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.secret()).into_string();

    let response = KeypairResponse {
        success: true,
        data: KeypairData { pubkey, secret },
    };

    Json(response)
}

// create token

#[derive(Deserialize)]
struct CreateTokenRequest {
    mint_authority: String,
    mint: String,
    decimals: u8,
}

#[derive(Serialize)]
struct AccountMetaJson {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[derive(Serialize)]
struct TokenInstructionData {
    program_id: String,
    accounts: Vec<AccountMetaJson>,
    instruction_data: String,
}

#[derive(Serialize)]
struct CreateTokenResponse {
    success: bool,
    data: TokenInstructionData,
}

async fn create_token(Json(req): Json<CreateTokenRequest>) -> impl IntoResponse {
    match mint_instruction_build(req) {
        Ok(response) => Json(response).into_response(),
        Err(err_msg) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "success": false, "error": err_msg })),
        )
            .into_response(),
    }
}

fn mint_instruction_build(req: CreateTokenRequest) -> Result<CreateTokenResponse, String> {
    let mint = req
        .mint
        .parse::<Pubkey>()
        .map_err(|_| "Invalid mint pubkey")?;
    let mint_authority = req
        .mint_authority
        .parse::<Pubkey>()
        .map_err(|_| "Invalid mintAuthority pubkey")?;

    let ix = initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint,
        &mint_authority,
        None,
        req.decimals,
    )
    .map_err(|e| format!("Failed to build instruction: {e:?}"))?;

    let accounts: Vec<AccountMetaJson> = ix
        .accounts
        .into_iter()
        .map(|a| AccountMetaJson {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();

    let instruction_data = general_purpose::STANDARD.encode(ix.data);

    Ok(CreateTokenResponse {
        success: true,
        data: TokenInstructionData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data,
        },
    })
}

// mint token

#[derive(Deserialize)]
struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

#[derive(Serialize)]
struct MintTokenResponse {
    success: bool,
    data: TokenInstructionData,
}

async fn mint_token(Json(req): Json<MintTokenRequest>) -> impl IntoResponse {
    match build_mint_to_instruction(req) {
        Ok(response) => Json(response).into_response(),
        Err(err) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "success": false, "error": err })),
        )
            .into_response(),
    }
}

fn build_mint_to_instruction(req: MintTokenRequest) -> Result<MintTokenResponse, String> {
    let mint = req
        .mint
        .parse::<Pubkey>()
        .map_err(|_| "Invalid mint pubkey")?;
    let destination = req
        .destination
        .parse::<Pubkey>()
        .map_err(|_| "Invalid destination pubkey")?;
    let authority = req
        .authority
        .parse::<Pubkey>()
        .map_err(|_| "Invalid authority pubkey")?;

    let ix = mint_to(
        &TOKEN_PROGRAM_ID,
        &mint,
        &destination,
        &authority,
        &[], // no multisig signers
        req.amount,
    )
    .map_err(|e| format!("Failed to create mint_to instruction: {e:?}"))?;

    let accounts = ix
        .accounts
        .into_iter()
        .map(|a| AccountMetaJson {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();

    let instruction_data = general_purpose::STANDARD.encode(ix.data);

    Ok(MintTokenResponse {
        success: true,
        data: TokenInstructionData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data,
        },
    })
}
