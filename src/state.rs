use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::error::ByteMolarError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DentalRecord {
    pub patient_pubkey: Pubkey,
    pub dentist_pubkey: Pubkey,
    pub timestamp: i64,
    pub diagnosis: String,
    pub treatment_plan: String,
    pub is_completed: bool,
}

impl DentalRecord {
    pub fn validate(&self) -> Result<(), ByteMolarError> {
        // Validate diagnosis length
        if self.diagnosis.is_empty() || self.diagnosis.len() > 1000 {
            return Err(ByteMolarError::InvalidPatientData);
        }

        // Validate treatment plan
        if self.treatment_plan.is_empty() || self.treatment_plan.len() > 1000 {
            return Err(ByteMolarError::InvalidTreatmentPlan);
        }

        // Validate timestamp
        if self.timestamp <= 0 {
            return Err(ByteMolarError::InvalidPatientData);
        }

        Ok(())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Clinic {
    pub authority: Pubkey,
    pub name: String,
    pub total_patients: u64,
    pub active_treatments: u64,
}

impl Clinic {
    pub fn validate(&self) -> Result<(), ByteMolarError> {
        // Validate clinic name
        if self.name.is_empty() || self.name.len() > 50 {
            return Err(ByteMolarError::InvalidClinicName);
        }

        // Validate counts
        if self.active_treatments > self.total_patients {
            return Err(ByteMolarError::InvalidPatientData);
        }

        Ok(())
    }

    pub fn increment_patients(&mut self) {
        self.total_patients = self.total_patients.saturating_add(1);
    }

    pub fn increment_active_treatments(&mut self) {
        self.active_treatments = self.active_treatments.saturating_add(1);
    }

    pub fn decrement_active_treatments(&mut self) {
        self.active_treatments = self.active_treatments.saturating_sub(1);
    }
}