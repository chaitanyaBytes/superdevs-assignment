use solana_sdk::{signature::Keypair, signer::Signer};

use crate::models::KeypairResponse;

pub fn create_keyair() -> Result<KeypairResponse, String> {
    let keypair = Keypair::new();

    let pubkey = keypair.pubkey().to_string();
    let secret = keypair.to_base58_string();

    Ok(KeypairResponse { pubkey, secret })
}
