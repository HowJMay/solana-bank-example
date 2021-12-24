use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum BankError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Not Rent Exempt")]
    NotRentExempt,
}

impl From<BankError> for ProgramError {
    fn from(e: BankError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
