//! State transition types
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use arrayref::{ array_ref, array_refs,array_mut_ref,mut_array_refs,};
/// Program states
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInfo {
    /// Initialized state
    pub is_initialized: bool,
    pub nonce: u8,
    /// Token A
    pub token_a: Pubkey,
    /// Token B
    pub token_b: Pubkey,
    /// Public key of the admin token account to receive fees
    pub admin_key: Pubkey,
    // Rate token A 
    pub rate_a: u64,
    // Rate token B
    pub rate_b: u64,
    /// Fee
    pub fee: u64
}

impl Sealed for SwapInfo {}
impl IsInitialized for SwapInfo {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for SwapInfo {
    const LEN: usize = 122;
    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, 122];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            is_initialized,
            nonce,
            token_a,
            token_b,
            admin_key,
            rate_a,
            rate_b,
            fee,
        ) = array_refs![input,1,1,32,32,32,8,8,8];
        Ok(Self {
            is_initialized: match is_initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            nonce: u8::from_le_bytes(*nonce),
            token_a: Pubkey::new_from_array(*token_a),
            token_b: Pubkey::new_from_array(*token_b),
            admin_key: Pubkey::new_from_array(*admin_key),
            rate_a: u64::from_le_bytes(*rate_a),
            rate_b:u64::from_le_bytes(*rate_b),
            fee:u64::from_le_bytes(*fee),
        })
    }
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output,0,122];
        let (
            is_initialized,
            nonce,
            token_a,
            token_b,
            admin_key,
            rate_a,
            rate_b,
            fee,
        ) = mut_array_refs![output,1,1,32,32,32,8,8,8];
        is_initialized[0] = self.is_initialized as u8;
        token_a.copy_from_slice(self.token_a.as_ref());
        token_b.copy_from_slice(self.token_b.as_ref());
        *rate_a = self.rate_a.to_be_bytes();
        *rate_b = self.rate_b.to_be_bytes();
        *fee = self.fee.to_be_bytes();
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_swap_info_packing() {
        let is_initialized = true;
        let token_a_raw = [1u8; 32];
        let token_b_raw = [2u8;32];
        let admin_key_raw = [3u8;32];
        let rate_a = 1u64;
        let rate_b = 2u64;
        let fee = 50u64;
        let nonce = 1u8;
        let token_a = Pubkey::new_from_array(token_a_raw);
        let token_b =  Pubkey::new_from_array(token_b_raw);
        let admin_key =  Pubkey::new_from_array(admin_key_raw);
        let swap_info = SwapInfo {
            is_initialized,
            nonce,
            token_a,
            token_b,
            admin_key,
            rate_a,
            rate_b,
            fee,
        };

        let mut packed = [0u8; SwapInfo::LEN];
        SwapInfo::pack(swap_info, &mut packed).unwrap();
        let unpacked = SwapInfo::unpack(&packed).unwrap();
        assert_eq!(swap_info, unpacked);
    }
}