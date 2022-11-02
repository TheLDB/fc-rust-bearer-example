// Crate Imports
use ethers::{
    prelude::k256::ecdsa::SigningKey,
    signers::Signer,
    signers::{coins_bip39::English, MnemonicBuilder, Wallet},
    types::Signature,
};

use serde_json::{json, Value};

use chrono::{DateTime, Utc};
#[tokio::main] // Make the main function an async runtime
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Return a boxed error for calls we want to unwrap

    // Create a new wallet using Ethers
    let wallet: Wallet<SigningKey> = MnemonicBuilder::<English>::default()
        .word_count(12)
        // Never hardcode recovery phrases or secret keys, they grant FULL access to your wallet
        .phrase("Farcaster Farcaster Farcaster Farcaster Farcaster Farcaster Farcaster Farcaster Farcaster Farcaster Farcaster Farcaster")
        .build()?;

    // Get the current unix timestamp (non-leap seconds since January 1, 1970 00:00:00 UTC)
    let dt: DateTime<Utc> = Utc::now();
    let timestamp: i64 = dt.timestamp();
    let expires_at: i64 = timestamp + 300; // Generate an expires_at 5 minutes in the future (300 seconds)

    // Initialize a bearer payload using serde_json
    let payload: Value = json!({
        "method": "generateToken",
        "params": {
            "timestamp": timestamp,
            "expiresAt": expires_at
        }
    });

    // Sign the payload using our ethers wallet
    let signature: Signature = wallet.sign_message(payload.to_string()).await?;

    // Convert the signature from String -> Vec<u8>
    let arrayify: Vec<u8> = hex::decode(signature.to_string())?;

    // Encode the signature to a base64 format
    let base64_signature = base64::encode(arrayify);

    // Format the signature to be a proper bearer token
    let bearer = format!("eip191:{}", base64_signature);

    println!("{}", bearer);

    Ok(())
}
