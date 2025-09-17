//! Error types

use solana_program_error::{ProgramError, ToStr};

/// Errors that may be returned by the interface.
#[repr(u32)]
#[derive(Clone, Debug, Eq, thiserror::Error, num_derive::FromPrimitive, PartialEq)]
pub enum TransferHookError {
    /// Incorrect account provided
    #[error("Incorrect account provided")]
    IncorrectAccount = 2_110_272_652,
    /// Mint has no mint authority
    #[error("Mint has no mint authority")]
    MintHasNoMintAuthority,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
    /// Program called outside of a token transfer
    #[error("Program called outside of a token transfer")]
    ProgramCalledOutsideOfTransfer,
}

impl From<TransferHookError> for ProgramError {
    fn from(e: TransferHookError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl ToStr for TransferHookError {
    fn to_str(&self) -> &'static str {
        match self {
            TransferHookError::IncorrectAccount => "Incorrect account provided",
            TransferHookError::MintHasNoMintAuthority => "Mint has no mint authority",
            TransferHookError::IncorrectMintAuthority => {
                "Incorrect mint authority has signed the instruction"
            }
            TransferHookError::ProgramCalledOutsideOfTransfer => {
                "Program called outside of a token transfer"
            }
        }
    }
}
