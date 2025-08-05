use std::str::FromStr;

use actix_web::web;
use base64::{Engine as _, engine::general_purpose};
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use spl_token::{
    ID as TOKEN_PROGRAM_ID,
    instruction::{initialize_mint, mint_to},
};

use crate::models::{
    AccountMetaModel, CreateTokenRequest, CreateTokenResponse, MintTokenRequest, MintTokenResponse,
};

pub fn create_initialize_mint_ix(
    body: web::Json<CreateTokenRequest>,
) -> Result<CreateTokenResponse, String> {
    let mint_authority = body
        .mintAuthority
        .as_ref()
        .ok_or_else(|| "Mint authority is missing".to_string())?;

    let mint = body.mint.as_ref().ok_or_else(|| "Mint is missing")?;

    let mint_authority_pubkey = Pubkey::from_str(&mint_authority)
        .map_err(|e| format!("Mint authority is not a valid pubkey {e}"))?;

    // validate token mint and parse it to pubkey
    let mint_pubkey =
        Pubkey::from_str(&mint).map_err(|e| format!("Mint is not a valid pubkey {e}"))?;

    let decimals = body.decimals;

    let initialize_mint_ix = initialize_mint(
        &TOKEN_PROGRAM_ID,
        &mint_pubkey,
        &mint_authority_pubkey,
        None,
        decimals,
    )
    .map_err(|e| format!("Failed to build transcation {}", e))?;

    let accounts = initialize_mint_ix
        .accounts
        .iter()
        .map(|a| AccountMetaModel {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();

    Ok(CreateTokenResponse {
        program_id: initialize_mint_ix.program_id.to_string(),
        accounts: accounts,
        instruction_data: general_purpose::STANDARD.encode(initialize_mint_ix.data),
    })
}

pub fn create_mint_token_ix(
    body: web::Json<MintTokenRequest>,
) -> Result<MintTokenResponse, String> {
    let mint = body
        .mint
        .as_ref()
        .ok_or_else(|| "Mint is missing".to_string())?;

    let destination = body
        .destination
        .as_ref()
        .ok_or_else(|| "destination address is missing".to_string())?;

    let authority = body
        .authority
        .as_ref()
        .ok_or_else(|| "authority is missing".to_string())?;

    let mint_pubkey =
        Pubkey::from_str(&mint).map_err(|e| format!("Mint is not a valid pubkey {e}"))?;

    let destination_pubkey = Pubkey::from_str(&destination)
        .map_err(|e| format!("destination is not a valid pubkey {e}"))?;

    let authority_pubkey =
        Pubkey::from_str(&authority).map_err(|e| format!("authority is not a valid pubkey {e}"))?;

    let amount = body.amount;

    let destination_ata = get_associated_token_address(&destination_pubkey, &mint_pubkey);

    let mint_to_ix = mint_to(
        &TOKEN_PROGRAM_ID,
        &mint_pubkey,
        &destination_ata,
        &authority_pubkey,
        &[],
        amount,
    )
    .map_err(|e| format!("failed to build transaction {}", e))?;

    let accounts = mint_to_ix
        .accounts
        .iter()
        .map(|a| AccountMetaModel {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();

    Ok(MintTokenResponse {
        program_id: mint_to_ix.program_id.to_string(),
        accounts: accounts,
        instruction_data: general_purpose::STANDARD.encode(mint_to_ix.data),
    })
}
