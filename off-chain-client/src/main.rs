// use anchor_client::Program;
use anchor_client::anchor_lang::prelude::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, read_keypair_file},
    signer::Signer,
    commitment_config::CommitmentConfig,
};
// use anyhow::Result;

const RPC_URL: &str = "http://127.0.0.1:8899"; // Localnet RPC URL
// const PROGRAM_ID: &str = "YourProgramID1111111111111111111111111111111111"; // Replace with your Anchor program ID
const PAYER_KEYPAIR_PATH: &str = "~/.config/solana/id.json"; // Replace with your keypair path

// #[tokio::main]
fn main() -> Result<()> {
    // 1. Set up the Solana RPC client
    let rpc_client = RpcClient::new_with_commitment(RPC_URL.to_string(), CommitmentConfig::confirmed());
    
    // 2. Load the payer's keypair
    let payer = read_keypair_file(shellexpand::tilde(PAYER_KEYPAIR_PATH).to_string())?;
    println!("Using payer: {}", payer.pubkey());

    // 3. Fetch all vote accounts
    let vote_accounts = rpc_client.get_vote_accounts()?.current;
    println!("Found {} vote accounts.", vote_accounts.len());

    // Extract their public keys
    let vote_account_pubkeys: Vec<Pubkey> = vote_accounts
        .iter()
        .map(|vote_account| Pubkey::from_str(&vote_account.vote_pubkey).unwrap())
        .collect();

    for item in &vote_account_pubkeys {
        println!("{:?}", item.to_bytes());
    }

    // // 4. Set up Anchor client
    // let wallet = anchor_client::solana_sdk::signature::Keypair::from_bytes(&payer.to_bytes())?;
    // let client = anchor_client::Client::new_with_options(
    //     anchor_client::Cluster::Localnet, // Use localnet RPC
    //     wallet,
    //     CommitmentConfig::confirmed(),
    // );

    // let program_id = Pubkey::from_str(PROGRAM_ID)?;
    // let program: Program = client.program(program_id)?;

    // // 5. Invoke the Anchor program's process_vote_accounts function
    // let tx_sig = program
    //     .request()
    //     .accounts(anchor_client::anchor_lang::ToAccountMetas::to_account_metas(
    //         &ProcessVoteAccounts { signer: payer.pubkey() },
    //         None,
    //     ))
    //     .args((vote_account_pubkeys,))
    //     .send()?;

    // println!("Transaction Signature: {}", tx_sig);

    Ok(())
}

// Struct matching the Anchor program's accounts
// #[derive(anchor_client::anchor_lang::Accounts)]
// pub struct ProcessVoteAccounts {
//     #[account(mut)]
//     pub signer: Pubkey,
// }