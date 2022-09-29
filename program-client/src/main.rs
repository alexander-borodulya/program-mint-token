use std::{
    str::FromStr,
    error::Error
};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    native_token::LAMPORTS_PER_SOL, 
    pubkey::Pubkey,
    instruction::Instruction,
};
use solana_transaction_status::TransactionConfirmationStatus;
use solana_sdk::{
    signer::Signer,
    transaction::Transaction
};

// From lib.rs
use program_mint::{
    create_keypair, 
    sleep_ms,
    print_balance,
    lamports_to_sol,
    MintInstruction,
};

// Global variables
const URL: &str = "https://api.devnet.solana.com";

fn main() -> Result<(), Box<dyn Error>> {
    let rpc_client = RpcClient::new(URL);
    let program_id = Pubkey::from_str("TODO: Deployed Hash for the solana program")?;
    let sender = create_keypair();
    println!("Sender/Payer: {:?}", sender.pubkey());
    
    //
    // TODO: Create static account instead of generating new keypair each time the client runs
    //

    // Check SOL ammout
    let sender_balance = rpc_client.get_balance(&sender.pubkey())?;
    println!("Sender/Payer balance (1): {:?}", lamports_to_sol(sender_balance));

    let airdrop_sol_amount = 1;
    let airdrop_lamports_amount = airdrop_sol_amount * LAMPORTS_PER_SOL;

    // // Airdrop SOL - Handle confirm_transaction
    // if let Ok(airdrop_sig) = rpc_client.request_airdrop(&sender.pubkey(), airdrop_lamports_amount) {
    //     println!("Airdrop sig: {:?}", airdrop_sig);

    //     let mut airdrop_confirm_count = 1;
    //     loop {
    //         if let Ok(airdrop_confirmed) = rpc_client.confirm_transaction(&airdrop_sig) {
    //             if airdrop_confirmed {
    //                 println!("Airdrop confirmed ({:?}) after {:?} confirmation(s)", airdrop_confirmed, airdrop_confirm_count);
    //                 break;
    //             }
    //         }
    //         airdrop_confirm_count += 1;
    //     }
    // }

    // Airdrop SOL - Handle confirmation_status
    let airdrop_sig = rpc_client.request_airdrop(&sender.pubkey(), airdrop_lamports_amount)
        .expect("Failed to request_airdrop");

    println!("Airdrop sig: {:?}", airdrop_sig);

    let mut airdrop_tx_attempt_count = 1;
    loop {
        println!("Airdrop about to confirm, atempt {}...", airdrop_tx_attempt_count);

        let status = loop {
            if let Ok(statuses) = rpc_client.get_signature_statuses(&[airdrop_sig]) {
                if let Some(status) = statuses.value[0].clone() {
                    break status;
                }
            }
            sleep_ms(250);
        };

        match status.confirmation_status.unwrap() {
            TransactionConfirmationStatus::Processed => {
                print_balance("Airdrop tx processed", &rpc_client, &sender.pubkey());
            }
            TransactionConfirmationStatus::Confirmed => {
                print_balance("Airdrop tx confirmed", &rpc_client, &sender.pubkey());
            },
            TransactionConfirmationStatus::Finalized => {
                print_balance("Airdrop tx finalized", &rpc_client, &sender.pubkey());
                break;
            }
        }
        airdrop_tx_attempt_count += 1;
    }

    // Check SOL ammout
    let sender_balance = rpc_client.get_balance(&sender.pubkey())?;
    println!("Sender / Payer balance (2): {:?}", lamports_to_sol(sender_balance));

    //
    // Execute program {
    //
    let latest_blockhash = rpc_client.get_latest_blockhash()?;
    println!("Latest block hash: {:?}", latest_blockhash);
        
    let mint_ix_init = MintInstruction::Init { 
        message: "Message from program-client".to_owned()
    };

    let ix = Instruction::new_with_borsh(
        program_id, 
        &mint_ix_init, 
        vec![], 
    );
    
    let ix_vec = vec!(ix);
    
    // Create transaction
    let mut tx = Transaction::new_with_payer(
        &ix_vec,
        Some(&sender.pubkey()),
    );
    
    tx.sign(
        &[&sender], 
        latest_blockhash
    );
    
    let tx_sig = rpc_client.send_and_confirm_transaction(&tx).expect("Transaction failed");
    println!("Transaction succeded: Signature: {:?}", tx_sig);
    
    Ok(())

    //
    // Execute program }
    //
}
