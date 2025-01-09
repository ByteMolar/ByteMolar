use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};

use crate::state::DentalRecord;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AddRecordArgs {
    pub diagnosis: String,
    pub treatment_plan: String,
}

pub fn process_add_record(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AddRecordArgs,
) -> ProgramResult {
    msg!("Processing AddRecord instruction");
    
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let record_account = next_account_info(account_info_iter)?;
    let patient = next_account_info(account_info_iter)?;
    let dentist = next_account_info(account_info_iter)?;

    // Create dental record
    let record = DentalRecord {
        patient_pubkey: *patient.key,
        dentist_pubkey: *dentist.key,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        diagnosis: args.diagnosis,
        treatment_plan: args.treatment_plan,
        is_completed: false,
    };

    msg!("Created new dental record");
    Ok(())
}