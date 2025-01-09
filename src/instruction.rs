use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ByteMolarInstruction {
    /// Creates a new dental clinic
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Clinic account
    /// 2. `[signer]` Authority
    /// 3. `[]` System program
    CreateClinic {
        name: String,
    },

    /// Adds a new dental record
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Record account
    /// 2. `[writable]` Clinic account
    /// 3. `[signer]` Authority
    /// 4. `[]` System program
    AddDentalRecord {
        diagnosis: String,
        treatment_plan: String,
    },
}

impl ByteMolarInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => Self::CreateClinic {
                name: String::try_from_slice(rest)?,
            },
            1 => {
                let diagnosis = String::try_from_slice(rest)?;
                let (rest, _) = rest.split_at(diagnosis.try_to_vec()?.len());
                let treatment_plan = String::try_from_slice(rest)?;
                Self::AddDentalRecord {
                    diagnosis,
                    treatment_plan,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.pack_len());
        match self {
            Self::CreateClinic { name } => {
                buf.push(0);
                buf.extend_from_slice(&name.try_to_vec().unwrap());
            }
            Self::AddDentalRecord { diagnosis, treatment_plan } => {
                buf.push(1);
                buf.extend_from_slice(&diagnosis.try_to_vec().unwrap());
                buf.extend_from_slice(&treatment_plan.try_to_vec().unwrap());
            }
        }
        buf
    }

    pub fn pack_len(&self) -> usize {
        match self {
            Self::CreateClinic { name } => 1 + name.len(),
            Self::AddDentalRecord { diagnosis, treatment_plan } => {
                1 + diagnosis.len() + treatment_plan.len()
            }
        }
    }
}

pub fn create_clinic(
    program_id: &Pubkey,
    clinic_account: &Pubkey,
    authority: &Pubkey,
    name: String,
) -> Instruction {
    let data = ByteMolarInstruction::CreateClinic { name }.pack();
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*clinic_account, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data,
    }
}

pub fn add_dental_record(
    program_id: &Pubkey,
    record_account: &Pubkey,
    clinic_account: &Pubkey,
    authority: &Pubkey,
    diagnosis: String,
    treatment_plan: String,
) -> Instruction {
    let data = ByteMolarInstruction::AddDentalRecord {
        diagnosis,
        treatment_plan,
    }.pack();
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*record_account, false),
            AccountMeta::new(*clinic_account, false),
            AccountMeta::new(*authority, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data,
    }
}