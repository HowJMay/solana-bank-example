use solana_program::{
    account_info::{AccountInfo},
    entrypoint::ProgramResult,
    msg,
    // program::{invoke, invoke_signed},
    // program_error::ProgramError,
    // program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    // sysvar::{rent::Rent, Sysvar},
};

use crate::{instruction::BankInstruction};

pub struct Processor;
impl Processor {
    pub fn process (
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = BankInstruction::unpack(instruction_data)?;

        match instruction {
            BankInstruction::Deposit {amount, proof} => {
                msg!("Instruction: Deposit");
                msg!("{}", proof);
                Self::process_deposit(program_id, accounts, amount)
            }

            BankInstruction::Withdraw {amount, proof} => {
                msg!("Instruction: Withdraw");
                msg!("{}", proof);
                Self::process_withdraw(program_id, accounts, amount)
            }
        }
    }

    pub fn process_deposit(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _amount: u64,
    ) -> ProgramResult {
        Ok(())
    }

    pub fn process_withdraw(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _amount: u64,
    ) -> ProgramResult{
        Ok(())
    }
}