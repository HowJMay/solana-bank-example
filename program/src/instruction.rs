use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::BankError;

pub enum BankInstruction {
    Deposit {
        amount: u64,
        proof: String,
    },
    Withdraw {
        amount: u64,
        proof: String,
    },
}

impl BankInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(BankError::InvalidInstruction)?;

        Ok(match tag {
            0=> Self::Deposit {
                amount: Self::unpack_amount(rest)?,
                proof: Self::unpack_proof(rest)?,
            },
            1 => Self::Withdraw {
                amount: Self::unpack_amount(rest)?,
                proof: Self::unpack_proof(rest)?,
            },
            _=>return Err(BankError::InvalidInstruction.into())
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(BankError::InvalidInstruction)?;

        Ok(amount)
    }

    fn unpack_proof(input: &[u8]) -> Result<String, ProgramError> {
        let buf = input.get(9..17).ok_or(BankError::InvalidInstruction)?;

        let proof = match String::from_utf8(buf.to_vec()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        Ok(proof)
    }
}
