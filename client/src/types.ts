export type ClusterType = 'mainnet-beta' | 'devnet' | 'localnet';

export interface ClinicData {
    authority: string;
    name: string;
    totalPatients: number;
    activeTreatments: number;
}

export interface DentalRecordData {
    patientPubkey: string;
    dentistPubkey: string;
    timestamp: number;
    diagnosis: string;
    treatmentPlan: string;
    isCompleted: boolean;
}

export interface TransactionResult {
    signature: string;
    success: boolean;
    error?: string;
}

export interface AccountMeta {
    pubkey: string;
    isSigner: boolean;
    isWritable: boolean;
}

export interface ProgramInstruction {
    programId: string;
    keys: AccountMeta[];
    data: Buffer;
}