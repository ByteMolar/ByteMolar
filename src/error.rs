use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum ByteMolarError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    
    #[error("Not Rent Exempt")]
    NotRentExempt,
    
    #[error("Unauthorized Access")]
    UnauthorizedAccess,
    
    #[error("Invalid Patient Data")]
    InvalidPatientData,

    #[error("Clinic Already Exists")]
    ClinicAlreadyExists,
    
    #[error("Clinic Not Found")]
    ClinicNotFound,
    
    #[error("Record Already Exists")]
    RecordAlreadyExists,
    
    #[error("Invalid Record Size")]
    InvalidRecordSize,
    
    #[error("Invalid Authority")]
    InvalidAuthority,
    
    #[error("Invalid Clinic Name")]
    InvalidClinicName,
    
    #[error("Invalid Treatment Plan")]
    InvalidTreatmentPlan,
    
    #[error("Insufficient Funds")]
    InsufficientFunds,
}

impl From<ByteMolarError> for ProgramError {
    fn from(e: ByteMolarError) -> Self {
        ProgramError::Custom(e as u32)
    }
}