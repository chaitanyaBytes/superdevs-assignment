use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub mintAuthority: Option<String>,
    pub mint: Option<String>,
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct AccountMetaModel {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaModel>,
    pub instruction_data: String,
}

#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: Option<String>,
    pub destination: Option<String>,
    pub authority: Option<String>,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct MintTokenResponse {
    pub program_id: String,
    pub accounts: Vec<AccountMetaModel>,
    pub instruction_data: String,
}
