use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};

use crate::state::Clinic;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateClinicArgs {
    pub name: String,
}

pub fn process_create_clinic(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateClinicArgs,
) -> ProgramResult {
    msg!("Processing CreateClinic instruction");
    
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let clinic_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    // Create clinic state
    let clinic = Clinic {
        authority: *authority.key,
        name: args.name,
        total_patients: 0,
        active_treatments: 0,
    };

    msg!("Created new clinic: {}", clinic.name);
    Ok(())
}