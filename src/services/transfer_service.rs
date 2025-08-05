use std::str::FromStr;

use actix_web::web;
use solana_sdk::{pubkey::Pubkey, system_instruction::transfer};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{ID as TOKEN_PROGRAM_ID, instruction::transfer as transfer_token};

use crate::models::{
    AccountMeta, SendSolRequest, SendSolResponse, SendTokenRequest, SendTokenResponse,
};

pub fn create_send_sol_ix(body: web::Json<SendSolRequest>) -> Result<SendSolResponse, String> {
    let from = body.from.clone().ok_or_else(|| "from is missing")?;
    let to = body.to.clone().ok_or_else(|| "to is missing")?;

    let from = Pubkey::from_str(&from).map_err(|_| format!("Invalid sender public key"))?;
    let to = Pubkey::from_str(&to).map_err(|e| format!("to key is invalid {}", e))?;
    let lamports = body.lamports;

    if lamports <= 0 {
        return Err("Amount must be greater than 0".to_string());
    }

    let send_sol_ix = transfer(&from, &to, lamports);

    let accounts = send_sol_ix
        .accounts
        .iter()
        .map(|a| a.pubkey.to_string())
        .collect();

    Ok(SendSolResponse {
        program_id: send_sol_ix.program_id.to_string(),
        accounts,
        instruction_data: bs58::encode(send_sol_ix.data).into_string(),
    })
}

pub fn create_send_token_ix(
    body: web::Json<SendTokenRequest>,
) -> Result<SendTokenResponse, String> {
    let destination = body
        .destination
        .clone()
        .ok_or_else(|| "destination is missing")?;
    let mint = body.mint.clone().ok_or_else(|| "mint is missing")?;
    let owner = body.owner.clone().ok_or_else(|| "owner is missing")?;

    let destination =
        Pubkey::from_str(&destination).map_err(|e| format!("destination key is invalid {}", e))?;
    let mint = Pubkey::from_str(&mint).map_err(|e| format!("mint is invalid {}", e))?;
    let owner = Pubkey::from_str(&owner).map_err(|e| format!("owner is invalid {}", e))?;
    let amount = body.amount;

    let reciver_ata = get_associated_token_address(&destination, &mint);

    let send_token_ix =
        transfer_token(&TOKEN_PROGRAM_ID, &owner, &reciver_ata, &owner, &[], amount)
            .map_err(|e| format!("failed to build ix: {e}"))?;

    let accounts = send_token_ix
        .accounts
        .iter()
        .map(|a| AccountMeta {
            pubkey: a.pubkey.to_string(),
            isSigner: a.is_signer,
        })
        .collect();

    Ok(SendTokenResponse {
        program_id: send_token_ix.program_id.to_string(),
        accounts,
        instruction_data: bs58::encode(send_token_ix.data).into_string(),
    })
}
