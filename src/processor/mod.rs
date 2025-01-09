mod validation;
use validation::*;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::{invoke, invoke_signed},
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::{DentalRecord, Clinic};
use crate::error::ByteMolarError;

pub struct Processor;

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = instruction_data[0];
        let data = &instruction_data[1..];

        match instruction {
            0 => Self::process_create_clinic(program_id, accounts, data),
            1 => Self::process_add_record(program_id, accounts, data),
            _ => Err(ByteMolarError::InvalidInstruction.into()),
        }
    }

    fn process_create_clinic(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let clinic_account = next_account_info(accounts_iter)?;
        let authority = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        // Add validation
        validate_clinic_account(program_id, clinic_account, authority)?;

        let clinic_data = Clinic::try_from_slice(data)?;
        clinic_data.validate()?;
        
        let rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(std::mem::size_of::<Clinic>());

        invoke(
            &system_instruction::create_account(
                authority.key,
                clinic_account.key,
                rent_lamports,
                std::mem::size_of::<Clinic>() as u64,
                program_id,
            ),
            &[
                authority.clone(),
                clinic_account.clone(),
                system_program.clone(),
            ],
        )?;

        clinic_data.serialize(&mut *clinic_account.data.borrow_mut())?;
        msg!("Created new clinic account");
        Ok(())
    }

    fn process_add_record(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let record_account = next_account_info(accounts_iter)?;
        let clinic_account = next_account_info(accounts_iter)?;
        let authority = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        // Add validation
        validate_record_account(program_id, record_account, clinic_account, authority)?;

        let record_data = DentalRecord::try_from_slice(data)?;
        record_data.validate()?;
        
        let rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(std::mem::size_of::<DentalRecord>());

        invoke(
            &system_instruction::create_account(
                authority.key,
                record_account.key,
                rent_lamports,
                std::mem::size_of::<DentalRecord>() as u64,
                program_id,
            ),
            &[
                authority.clone(),
                record_account.clone(),
                system_program.clone(),
            ],
        )?;

        record_data.serialize(&mut *record_account.data.borrow_mut())?;
        msg!("Added new dental record");
        Ok(())
    }
}