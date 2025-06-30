use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use base64::{Engine as _, engine::general_purpose};
use bs58;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use solana_program::pubkey::Pubkey;
use solana_pubkey::Pubkey as Pubkeyy;
use solana_sdk::{
    instruction::Instruction,
    signature::Signature,
    signature::{Keypair, Signer},
    system_instruction,
};
use spl_token::{
    ID as TOKEN_PROGRAM_ID,
    instruction::{initialize_mint, mint_to, transfer},
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/keypair", post(keygen))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        // .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token));

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

pub async fn keygen() -> impl IntoResponse {
    let keypair = Keypair::new();

    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

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
        .parse::<Pubkeyy>()
        .map_err(|_| "Invalid mint pubkey")?;
    let mint_authority = req
        .mint_authority
        .parse::<Pubkeyy>()
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
        .parse::<Pubkeyy>()
        .map_err(|_| "Invalid mint pubkey")?;
    let destination = req
        .destination
        .parse::<Pubkeyy>()
        .map_err(|_| "Invalid destination pubkey")?;
    let authority = req
        .authority
        .parse::<Pubkeyy>()
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

// Sign message
#[derive(Deserialize)]
struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
struct SignMessageResponse {
    success: bool,
    data: Option<SignedData>,
    error: Option<String>,
}

#[derive(Serialize)]
struct SignedData {
    signature: String,
    public_key: String,
    message: String,
}

async fn sign_message(Json(req): Json<SignMessageRequest>) -> impl IntoResponse {
    if req.message.is_empty() || req.secret.is_empty() {
        return Json(SignMessageResponse {
            success: false,
            data: None,
            error: Some("Missing required fields".to_string()),
        });
    }

    // Decode base58 secret to 64-byte keypair
    let secret_bytes = match bs58::decode(&req.secret).into_vec() {
        Ok(bytes) if bytes.len() == 64 => bytes,
        _ => {
            return Json(SignMessageResponse {
                success: false,
                data: None,
                error: Some("Invalid secret key".to_string()),
            });
        }
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Json(SignMessageResponse {
                success: false,
                data: None,
                error: Some("Failed to parse secret key".to_string()),
            });
        }
    };

    let message_bytes = req.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);
    let signature_b64 = general_purpose::STANDARD.encode(signature.as_ref());

    Json(SignMessageResponse {
        success: true,
        data: Some(SignedData {
            signature: signature_b64,
            public_key: keypair.pubkey().to_string(),
            message: req.message,
        }),
        error: None,
    })
}

// verify message

#[derive(Deserialize)]
struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

#[derive(Serialize)]
struct VerifyMessageResponse {
    success: bool,
    data: VerifyResult,
}

#[derive(Serialize)]
struct VerifyResult {
    valid: bool,
    message: String,
    pubkey: String,
}

// async fn verify_message(Json(req): Json<VerifyMessageRequest>) -> impl IntoResponse {
//     let pubkey = match req.pubkey.parse::<Pubkeyy>() {
//         Ok(p) => p,
//         Err(_) => {
//             return Json(serde_json::json!({
//                 "success": false,
//                 "error": "Invalid public key"
//             }));
//         }
//     };

//     let signature_bytes = match general_purpose::STANDARD.decode(&req.signature) {
//         Ok(bytes) => bytes,
//         Err(_) => {
//             return Json(serde_json::json!({
//                 "success": false,
//                 "error": "Invalid base64 signature"
//             }));
//         }
//     };

//     let signature = match Signature::try_from(signature_bytes.as_slice()) {
//         Ok(sig) => sig,
//         Err(_) => {
//             return Json(serde_json::json!({
//                 "success": false,
//                 "error": "Malformed signature"
//             }));
//         }
//     };

//     let message_bytes = req.message.as_bytes();

//     let is_valid = match signature.verify(&pubkey, message_bytes) {
//         Ok(_) => true,
//         Err(_) => false,
//     };

//     Json(VerifyMessageResponse {
//         success: true,
//         data: VerifyResult {
//             valid: is_valid,
//             message: req.message,
//             pubkey: req.pubkey,
//         },
//     })
// }

// send sol

#[derive(Deserialize)]
struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Serialize)]
struct SendSolResponse {
    success: bool,
    data: Option<TransferData>,
    error: Option<String>,
}

#[derive(Serialize)]
struct TransferData {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}

async fn send_sol(Json(req): Json<SendSolRequest>) -> impl IntoResponse {
    // Basic validations
    if req.lamports == 0 {
        return Json(SendSolResponse {
            success: false,
            data: None,
            error: Some("lamports must be greater than 0".to_string()),
        });
    }

    let from = match req.from.parse::<Pubkey>() {
        Ok(p) => p,
        Err(_) => {
            return Json(SendSolResponse {
                success: false,
                data: None,
                error: Some("Invalid 'from' address".to_string()),
            });
        }
    };

    let to = match req.to.parse::<Pubkey>() {
        Ok(p) => p,
        Err(_) => {
            return Json(SendSolResponse {
                success: false,
                data: None,
                error: Some("Invalid 'to' address".to_string()),
            });
        }
    };

    let instruction: Instruction = system_instruction::transfer(&from, &to, req.lamports);

    Json(SendSolResponse {
        success: true,
        data: Some(TransferData {
            program_id: instruction.program_id.to_string(),
            accounts: instruction
                .accounts
                .iter()
                .map(|a| a.pubkey.to_string())
                .collect(),
            instruction_data: general_purpose::STANDARD.encode(instruction.data),
        }),
        error: None,
    })
}

// send token
#[derive(Deserialize)]
struct TokenTransferRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

#[derive(Serialize)]
struct TokenTransferResponse {
    success: bool,
    data: Option<TransferInstructionData>,
    error: Option<String>,
}

#[derive(Serialize)]
struct TransferInstructionData {
    program_id: String,
    accounts: Vec<AccountMetaJson>,
    instruction_data: String,
}

async fn send_token(Json(req): Json<TokenTransferRequest>) -> impl IntoResponse {
    let owner = match req.owner.parse::<Pubkeyy>() {
        Ok(p) => p,
        Err(_) => return Json(error_response("Invalid owner address")),
    };

    let destination = match req.destination.parse::<Pubkeyy>() {
        Ok(p) => p,
        Err(_) => return Json(error_response("Invalid destination address")),
    };

    let mint = match req.mint.parse::<Pubkey>() {
        Ok(p) => p,
        Err(_) => return Json(error_response("Invalid mint address")),
    };

    if req.amount == 0 {
        return Json(error_response("Amount must be greater than 0"));
    }

    // Create instruction
    let ix = match transfer(
        &TOKEN_PROGRAM_ID,
        &owner,       // from_token_account
        &destination, // to_token_account
        &owner,       // authority
        &[],          // multisig
        req.amount,
    ) {
        Ok(ix) => ix,
        Err(e) => {
            return Json(error_response(&format!(
                "Failed to create instruction: {e:?}"
            )));
        }
    };

    let accounts = ix
        .accounts
        .iter()
        .map(|meta| AccountMetaJson {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();

    Json(TokenTransferResponse {
        success: true,
        data: Some(TransferInstructionData {
            program_id: ix.program_id.to_string(),
            accounts,
            instruction_data: general_purpose::STANDARD.encode(ix.data),
        }),
        error: None,
    })
}

fn error_response(msg: &str) -> TokenTransferResponse {
    TokenTransferResponse {
        success: false,
        data: None,
        error: Some(msg.to_string()),
    }
}
