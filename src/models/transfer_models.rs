use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: Option<String>,
    pub to: Option<String>,
    pub lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: Option<String>,
    pub mint: Option<String>,
    pub owner: Option<String>,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct AccountMeta {
    pub pubkey: String,
    pub isSigner: bool,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMeta>,
    pub instruction_data: String,
}
