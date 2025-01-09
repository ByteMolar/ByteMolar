use solana_program::{
    pubkey::Pubkey,
    system_instruction,
    hash::Hash,
};
use solana_program_test::*;
use solana_sdk::{
    signature::Keypair,
    transaction::Transaction,
    signer::Signer,
    transport::TransportError,
};
use borsh::BorshSerialize;

use crate::state::{Clinic, DentalRecord};

pub fn program_test(program_id: Pubkey) -> ProgramTest {
    let mut program_test = ProgramTest::new(
        "bytemolar",
        program_id,
        processor!(bytemolar::processor::process_instruction),
    );
    program_test.set_compute_max_units(100_000);
    program_test
}

pub async fn create_clinic(
    banks_client: &mut BanksClient,
    program_id: &Pubkey,
    payer: &Keypair,
    recent_blockhash: &Hash,
    clinic_account: &Keypair,
    name: String,
) -> Result<(), TransportError> {
    let clinic = Clinic {
        authority: payer.pubkey(),
        name,
        total_patients: 0,
        active_treatments: 0,
    };

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &clinic_account.pubkey(),
                100000000, // lamports
                1000,      // space
                program_id,
            ),
            // Add create clinic instruction
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, clinic_account], *recent_blockhash);

    banks_client.process_transaction(transaction).await?;
    Ok(())
}

pub async fn add_dental_record(
    banks_client: &mut BanksClient,
    program_id: &Pubkey,
    payer: &Keypair,
    recent_blockhash: &Hash,
    record_account: &Keypair,
    clinic_account: &Pubkey,
    patient_pubkey: Pubkey,
    diagnosis: String,
    treatment_plan: String,
) -> Result<(), TransportError> {
    let record = DentalRecord {
        patient_pubkey,
        dentist_pubkey: payer.pubkey(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        diagnosis,
        treatment_plan,
        is_completed: false,
    };

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &record_account.pubkey(),
                100000000, // lamports
                1000,      // space
                program_id,
            ),
            // Add dental record instruction
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, record_account], *recent_blockhash);

    banks_client.process_transaction(transaction).await?;
    Ok(())
}

pub fn create_test_clinic() -> Clinic {
    Clinic {
        authority: Pubkey::new_unique(),
        name: "Test Clinic".to_string(),
        total_patients: 0,
        active_treatments: 0,
    }
}

pub fn create_test_record() -> DentalRecord {
    DentalRecord {
        patient_pubkey: Pubkey::new_unique(),
        dentist_pubkey: Pubkey::new_unique(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        diagnosis: "Test Diagnosis".to_string(),
        treatment_plan: "Test Treatment Plan".to_string(),
        is_completed: false,
    }
}