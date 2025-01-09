use solana_program::{
    pubkey::Pubkey,
    system_instruction,
};
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

mod helpers;

#[tokio::test]
async fn test_create_clinic() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = 
        helpers::program_test(program_id).start().await;

    // Create test accounts
    let clinic_account = Keypair::new();
    let clinic_name = "Test Dental Clinic".to_string();

    // Create clinic
    helpers::create_clinic(
        &mut banks_client,
        &program_id,
        &payer,
        &recent_blockhash,
        &clinic_account,
        clinic_name.clone(),
    )
    .await
    .unwrap();

    // Verify clinic was created
    let clinic = banks_client
        .get_account(clinic_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(clinic.owner, program_id);
}

#[tokio::test]
async fn test_add_dental_record() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = 
        helpers::program_test(program_id).start().await;

    // Create test accounts
    let clinic_account = Keypair::new();
    let record_account = Keypair::new();
    let patient = Keypair::new();

    // Create clinic first
    helpers::create_clinic(
        &mut banks_client,
        &program_id,
        &payer,
        &recent_blockhash,
        &clinic_account,
        "Test Clinic".to_string(),
    )
    .await
    .unwrap();

    // Add dental record
    helpers::add_dental_record(
        &mut banks_client,
        &program_id,
        &payer,
        &recent_blockhash,
        &record_account,
        &clinic_account.pubkey(),
        patient.pubkey(),
        "Test Diagnosis".to_string(),
        "Test Treatment Plan".to_string(),
    )
    .await
    .unwrap();

    // Verify record was created
    let record = banks_client
        .get_account(record_account.pubkey())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(record.owner, program_id);
}

#[tokio::test]
async fn test_unauthorized_access() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = 
        helpers::program_test(program_id).start().await;

    // Create test accounts
    let unauthorized_user = Keypair::new();
    let clinic_account = Keypair::new();

    // Attempt to create clinic with unauthorized user
    let result = helpers::create_clinic(
        &mut banks_client,
        &program_id,
        &unauthorized_user, // Using unauthorized user
        &recent_blockhash,
        &clinic_account,
        "Test Clinic".to_string(),
    )
    .await;

    // Verify the transaction failed
    assert!(result.is_err());
}