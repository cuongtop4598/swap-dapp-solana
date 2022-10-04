//! Program entrypoint definitions

#![cfg(not(feature = "no-entrypoint"))]

use crate::{error::SwapError, processor::Processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey, msg,
};

entrypoint!(process_instruction);

fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process_swap_instruction(program_id, accounts, instruction_data) {
        return Err(error);
    }
    Ok(())
}