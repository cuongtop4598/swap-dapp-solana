use std::ops::Not;

use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey, 
    account_info::{AccountInfo, next_account_info}, 
    program_pack::Pack, 
    program_error::ProgramError,
    program::invoke_signed, msg
};
use crate::{utils, instruction::{SwapInstruction,InitializeData,SwapData}};
use crate::{state::SwapInfo, error::SwapError};
/// Program state handler. 
pub struct Processor {}

impl Processor {
    pub fn token_transfer<'a>(
        swap: &Pubkey,
        token_program: AccountInfo<'a>,
        source: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        nonce: u8,
        amount: u64
    ) -> Result<(), ProgramError> {
        let swap_bytes = swap.to_bytes();
        let authority_signature_seeds = [&swap_bytes[..32], &[nonce]];
        let signers = &[&authority_signature_seeds[..]];
        let ix = spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?;

        invoke_signed(
            &ix,
            &[source, destination, authority, token_program],
            signers
        )
    }
    pub fn process_initialize(
        nonce: u8,
        rate_a: u64,
        rate_b: u64,
        fee: u64,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        
        let swap_info = next_account_info(account_info_iter)?;
        let admin_key_info = next_account_info(account_info_iter)?;
        let token_a_info = next_account_info(account_info_iter)?;
        let token_b_info = next_account_info(account_info_iter)?;
        
        let token_swap = SwapInfo::unpack_unchecked(&swap_info.data.borrow())?;
        
        if token_swap.is_initialized {
            return Err(SwapError::AlreadyInUse.into());
        };

        let obj = SwapInfo {
            is_initialized: true,
            nonce,
            token_a: *token_a_info.key,
            token_b: *token_b_info.key,
            admin_key: *admin_key_info.key,
            rate_a,
            rate_b,
            fee
        };
        SwapInfo::pack(obj, &mut swap_info.data.borrow_mut())?;
        Ok(())
    }

    pub fn process_swap(
        program_id: &Pubkey,
        amount_in: u64,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        
        let swap_info = next_account_info(account_info_iter)?; // thông tin đặc điểm của pool 
        let authority_info = next_account_info(account_info_iter)?; // thông tin chương trình -> địa chỉ program thực thi instrution 
        let admin_info = next_account_info(account_info_iter)?; // địa chỉ nhận phí
        let swap_source_info = next_account_info(account_info_iter)?; // địa chỉ chứa token
        let swap_destination_info = next_account_info(account_info_iter)?; // địa chỉ người đổi SOL lấy token
        let token_program_info = next_account_info(account_info_iter)?; 
        let token_swap =  SwapInfo::unpack_unchecked(&swap_info.data.borrow())?;
        
        let token_amount = amount_in * u64::from(token_swap.rate_a) / u64::from(token_swap.rate_b);
        
        if *authority_info.key != utils::authority_id(program_id, swap_info.key, token_swap.nonce)?
        {
            return Err(SwapError::InvalidProgramAddress.into());
        }

        solana_program::system_instruction::transfer(
            swap_destination_info.key, 
            swap_source_info.key, 
            amount_in
            );
        
        Self::token_transfer(
            swap_info.key, 
            token_program_info.clone(), 
            swap_source_info.clone(), 
            swap_destination_info.clone(),
            authority_info.clone(),
            token_swap.nonce,
            token_amount
        )?;

        Self::token_transfer(
            swap_info.key, 
            token_program_info.clone(),
            swap_destination_info.clone(),
            admin_info.clone(),
            authority_info.clone(),
            token_swap.nonce,
            token_swap.fee,
    )?;
        Ok(())
    }

    pub fn process_swap_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = SwapInstruction::unpack(input)?;
         match instruction {
            SwapInstruction::Initialize(InitializeData {
                nonce,
                rate_a,
                rate_b,
                fee
            }) => {
                msg!("Instruction: Init");
                Self::process_initialize( nonce, rate_a, rate_b, fee, accounts);
            }
            SwapInstruction::Swap(SwapData {
                amount_in 
            }) => {
                msg!("Swap token in process");
                Self::process_swap(program_id, amount_in, accounts);
            }
        }
        Ok(())
    }

}