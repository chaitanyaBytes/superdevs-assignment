use std::str::FromStr;

use actix_web::web;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
};

use crate::models::{
    SignMessageRequest, SignMessageResponse, VerfiySignatureRequest, VerfiySignatureResponse,
};

pub fn sign_message_ix(body: web::Json<SignMessageRequest>) -> Result<SignMessageResponse, String> {
    let message = body
        .message
        .clone()
        .ok_or_else(|| "Missing required fields")?;

    let secret = body
        .secret
        .clone()
        .ok_or_else(|| "Missing required fields")?;

    let secret_bytes = bs58::decode(&secret)
        .into_vec()
        .map_err(|_| "secret is not a valid base58 string")?;

    if secret_bytes.len() != 64 {
        return Err("Invalid secret key length".to_string());
    }

    let keypair = Keypair::from_base58_string(&secret);

    let pubkey = keypair.pubkey().to_string();
    let signature = keypair.sign_message(message.as_bytes());

    Ok(SignMessageResponse {
        signature: signature.to_string(),
        pubkey: pubkey,
        message: message,
    })
}

pub fn verify_message_ix(
    body: web::Json<VerfiySignatureRequest>,
) -> Result<VerfiySignatureResponse, String> {
    let message = body.message.clone().ok_or_else(|| "Message is missing")?;
    let sign = body
        .signature
        .clone()
        .ok_or_else(|| "signature is missing")?;
    let pubkey = body.pubkey.clone().ok_or_else(|| "pubkey is missing")?;

    let pubkey = Pubkey::from_str(&pubkey).map_err(|e| format!("Not a valid pubkey {}", e))?;

    let signature = Signature::from_str(&sign).map_err(|_| "the signature is not of valid type")?;
    let is_valid_signature = signature.verify(pubkey.to_bytes().as_ref(), message.as_bytes());

    Ok(VerfiySignatureResponse {
        valid: is_valid_signature,
        message: message,
        pubkey: pubkey.to_string(),
    })
}
