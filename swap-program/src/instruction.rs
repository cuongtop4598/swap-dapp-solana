
use solana_program::{
    instruction::{Instruction, AccountMeta},
    program_error::ProgramError, pubkey::Pubkey,
};
use crate::error::SwapError;
use std::mem::size_of;

/// Initialize instruction data
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeData {
    pub nonce: u8,
    // Rate token A 
    pub rate_a: u64,
    // Rate token B
    pub rate_b: u64,
    /// Fee
    pub fee: u64
}

/// Swap instruction data
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct SwapData { 
    pub amount_in: u64,
}



#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SwapInstruction {
    /// Initializes a new Swap
    Initialize(InitializeData),
    // Swap token
    Swap(SwapData)
}

impl SwapInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(SwapError::InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (&nonce, rest) = rest.split_first().ok_or(SwapError::InvalidInstruction)?;
                let (rate_a, rest) = unpack_u64(rest)?;
                let (rate_b, rest) = unpack_u64(rest)?;
                let (fee, _rest) = unpack_u64(rest)?;
                Self::Initialize(InitializeData{
                    nonce,
                    rate_a,
                    rate_b,
                    fee
                })
            } 
            1 => {
                let (amount_in, _rest) = unpack_u64(rest)?;
                Self::Swap(SwapData {
                    amount_in,
                })
            } 
            _ => return Err(SwapError::InvalidInstruction.into()),
        })
    }
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match *self {
            Self::Initialize(InitializeData {
                nonce,
                rate_a,
                rate_b,
                fee
            }) => {
                buf.push(0); // add tag
                buf.push(nonce);
                buf.extend_from_slice(&rate_a.to_le_bytes());
                buf.extend_from_slice(&rate_b.to_le_bytes());
                buf.extend_from_slice(&fee.to_le_bytes());
            }
            Self::Swap(SwapData{ amount_in }) => {
                buf.push(1); // add tag
                buf.extend_from_slice(&amount_in.to_le_bytes());
            }
        }
       buf 
    }
    pub fn swap(
        program_id: &Pubkey,
        token_program_id: &Pubkey,
        source_pubkey: &Pubkey,
        destination_pubkey: &Pubkey,
        amount_in: u64,
    ) -> Result<Instruction, ProgramError> {
        let data = SwapInstruction::Swap(SwapData{amount_in}).pack();

        let accounts = vec![
            AccountMeta::new(*source_pubkey, false),
            AccountMeta::new(*destination_pubkey,false),
            AccountMeta::new(*token_program_id,false),
        ];
        Ok(Instruction { program_id: *program_id, accounts, data })
    }
} 

fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
    if input.len() >= 8 {
        let (amount, rest) = input.split_at(8);
        let amount = amount
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(SwapError::InvalidInstruction)?;
        Ok((amount, rest))
    } else {
        Err(SwapError::InvalidInstruction.into())
    }
}
