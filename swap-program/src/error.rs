//! Error types
use thiserror::Error;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
/// Errors that may be returned by the Swap program.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum SwapError { 
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Already in use")]
    AlreadyInUse,
    #[error("Expected account")]
    ExpectedAccount,
    #[error("Invalid program address")]
    InvalidProgramAddress
}

impl From<SwapError> for ProgramError {
    fn from(e: SwapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for SwapError {
    fn type_of() -> &'static str {
        "Swap Error"
    }
}