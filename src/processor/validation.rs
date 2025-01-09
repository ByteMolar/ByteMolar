use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::error::ByteMolarError;

pub fn validate_clinic_account(
    program_id: &Pubkey,
    clinic_account: &AccountInfo,
    authority: &AccountInfo,
) -> Result<(), ProgramError> {
    // Check account ownership
    if clinic_account.owner != program_id {
        return Err(ByteMolarError::InvalidAuthority.into());
    }

    // Validate authority is a signer
    if !authority.is_signer {
        return Err(ByteMolarError::UnauthorizedAccess.into());
    }

    // Check rent exemption
    if !Rent::get()?.is_exempt(clinic_account.lamports(), clinic_account.data_len()) {
        return Err(ByteMolarError::NotRentExempt.into());
    }

    Ok(())
}

pub fn validate_record_account(
    program_id: &Pubkey,
    record_account: &AccountInfo,
    clinic_account: &AccountInfo,
    authority: &AccountInfo,
) -> Result<(), ProgramError> {
    // Check account ownership
    if record_account.owner != program_id {
        return Err(ByteMolarError::InvalidAuthority.into());
    }

    // Validate authority is a signer
    if !authority.is_signer {
        return Err(ByteMolarError::UnauthorizedAccess.into());
    }

    // Check clinic account ownership
    if clinic_account.owner != program_id {
        return Err(ByteMolarError::ClinicNotFound.into());
    }

    // Check rent exemption
    if !Rent::get()?.is_exempt(record_account.lamports(), record_account.data_len()) {
        return Err(ByteMolarError::NotRentExempt.into());
    }

    Ok(())
}

pub fn validate_account_size(
    data_len: usize,
    min_size: usize,
    max_size: usize
) -> Result<(), ProgramError> {
    if data_len < min_size || data_len > max_size {
        return Err(ByteMolarError::InvalidRecordSize.into());
    }
    Ok(())
}