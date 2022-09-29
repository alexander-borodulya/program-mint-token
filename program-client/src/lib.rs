use std::time::Duration;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, native_token::LAMPORTS_PER_SOL};
use solana_sdk::{
    signature::{
        Keypair,
    }
};

// Instruction: Should be the same as in the program_solana
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum MintInstruction {
    Init { message: String },
}

//
// Utility functions
//
pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn sleep_ms(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}

pub fn print_balance(tag: &str, rpc_client: &RpcClient, pk: &Pubkey) {
    if let Ok(balance) = rpc_client.get_balance(pk) {
        println!("{}: balance: {} SOL", tag, lamports_to_sol(balance));
    } else {
        println!("print_balance: Failed to get balance of {:?}", pk);
    }
}

pub fn lamports_to_sol(l: u64) -> f64 {
    (l as f64) / LAMPORTS_PER_SOL as f64
}
