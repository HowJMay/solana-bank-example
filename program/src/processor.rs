use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};


use crate::{instruction::BankInstruction};

use crate::error::BankError;

pub struct Processor;
impl Processor {
    pub fn process (
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = BankInstruction::unpack(instruction_data)?;

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
        let account_info_iter = &mut _accounts.iter();

        let token_receiver = next_account_info(account_info_iter)?;
        let deposited_account = next_account_info(account_info_iter)?;
        let payer = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        
        // check rent exempt
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        if !rent.is_exempt(token_receiver.lamports(), token_receiver.data_len()) {
            return Err(BankError::NotRentExempt.into());
        }

        // generate a PDA, the PDA is going to own all the input accounts which contain the credited token(money) 
        let (pda, _nonce) = Pubkey::find_program_address(&[b"bank store"], _program_id);
        
        msg!("Calling the token program to transfer ownership to the Bank program");
        // set authority to the PDA
        let ownship_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            deposited_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            payer.key,
            &[&payer.key],
        )?;

        invoke(
            &ownship_change_ix,
            &[
                token_program.clone(),
                deposited_account.clone(),
                payer.clone(),
            ]
        )?;
        
        Ok(())
    }

    pub fn process_withdraw(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _amount: u64,
    ) -> ProgramResult{
        // check whether the note is valid

        // transmit money from PDA to receiver
        let account_info_iter = &mut _accounts.iter();

        let withdraw_account = next_account_info(account_info_iter)?;
        let deposited_account = next_account_info(account_info_iter)?;
        let pda_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // check rent exempt
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        if !rent.is_exempt(withdraw_account.lamports(), withdraw_account.data_len()) {
            return Err(BankError::NotRentExempt.into());
        }

        // generate a PDA, the PDA is going to own all the input accounts which contain the credited token(money) 
        let (pda, _nonce) = Pubkey::find_program_address(&[b"bank store"], _program_id);
        
        msg!("transfer token to the token receiver");
        let transfer_to_receiver_ix = spl_token::instruction::transfer(
            token_program.key,
            deposited_account.key,
            withdraw_account.key,
            &pda,
            &[&pda],
            _amount,
        )?;
        
        invoke_signed(
            &transfer_to_receiver_ix,
            &[
                token_program.clone(),
                deposited_account.clone(),
                withdraw_account.clone(),
                pda_account.clone(),
            ],
            &[&[&b"bank store"[..], &[_nonce]]],
        )?;

        Ok(())
    }
}