use solana_program::{
    account_info::{next_account_info, AccountInfo},
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

        let account_info_iter = &mut accounts.iter();
        let money_receiver = next_account_info(account_info_iter)?;
        msg!("money_receiver = {}", *money_receiver.key);

        match instruction {
            BankInstruction::Deposit {amount, note} => {
                msg!("Instruction: Deposit, amount: {}", amount);
                msg!("note: {}", note);
                Self::process_deposit(program_id, accounts, amount)
            }

            BankInstruction::Withdraw {amount, note} => {
                msg!("Instruction: Withdraw, amount: {}", amount);
                msg!("note: {}", note);
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