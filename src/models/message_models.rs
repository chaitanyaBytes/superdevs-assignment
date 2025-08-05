use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: Option<String>,
    pub secret: Option<String>,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub pubkey: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct VerfiySignatureRequest {
    pub message: Option<String>,
    pub signature: Option<String>,
    pub pubkey: Option<String>,
}

#[derive(Serialize)]
pub struct VerfiySignatureResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}
