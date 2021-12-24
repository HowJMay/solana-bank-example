use solana_program::{
    program_error::ProgramError,
};
use std::convert::TryInto;

use crate::error::BankError;

pub enum BankInstruction {
    Deposit {
        amount: u64,
        note: String,
    },
    Withdraw {
        amount: u64,
        note: String,
    },
}

impl BankInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(BankError::InvalidInstruction)?;
        Ok(match tag {
            0=> Self::Deposit {
                amount: Self::unpack_amount(rest)?,
                note: Self::unpack_note(rest)?,
            },
            1 => Self::Withdraw {
                amount: Self::unpack_amount(rest)?,
                note: Self::unpack_note(rest)?,
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

    fn unpack_note(input: &[u8]) -> Result<String, ProgramError> {
        let note_len = 12;
        let note_end = note_len + 8;
        let buf = input.get(8..note_end).ok_or(BankError::InvalidInstruction)?;
        let note = match String::from_utf8(buf.to_vec()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        Ok(note)
    }
}
