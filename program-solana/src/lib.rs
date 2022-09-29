use borsh::{
    BorshSerialize,
    BorshDeserialize
};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg, 
};

// Instruction: Should be the same as in the program_client
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum MintInstruction {
    Init { message: String },
}

entrypoint!(program_entrypoint);

pub fn program_entrypoint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions: &[u8],
) -> ProgramResult {
    msg!("program_entrypoint: program_id: {:?}, accounts len: {}, instructions len: {}", program_id, accounts.len(), instructions.len());
    Ok(())
}
