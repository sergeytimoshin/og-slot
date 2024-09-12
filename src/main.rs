use std::{env, str::FromStr};

use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

fn get_program_deployment_slot(rpc_url: &str, program_id: &Pubkey) -> Result<u64> {
    let client = RpcClient::new(rpc_url);

    // Get the transaction signatures for the program
    let signatures = client.get_signatures_for_address(program_id)?;

    if let Some(oldest_signature) = signatures.last() {
        let signature = Signature::from_str(&oldest_signature.signature)?;

        let config = solana_client::rpc_config::RpcTransactionConfig {
            encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
            commitment: None,
            max_supported_transaction_version: Some(0),
        };

        let transaction = client.get_transaction_with_config(&signature, config)?;

        // Return the slot number
        Ok(transaction.slot)
    } else {
        anyhow::bail!("No transactions found for the given program ID")
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        anyhow::bail!("Usage: {} <rpc_url> <program_id>", args[0]);
    }
    let rpc_url = &args[1];
    let program_id_str = &args[2];
    let program_id = Pubkey::from_str(program_id_str)
        .with_context(|| format!("Failed to parse program ID: {}", program_id_str))?;

    match get_program_deployment_slot(rpc_url, &program_id) {
        Ok(slot) => println!("{}", slot),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
