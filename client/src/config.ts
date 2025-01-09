import { ClusterType } from './types';

export interface NetworkConfig {
    cluster: ClusterType;
    endpoint: string;
    programId: string;
}

export const NETWORKS = {
    mainnet: {
        cluster: 'mainnet-beta' as ClusterType,
        endpoint: 'https://api.mainnet-beta.solana.com',
        programId: process.env.MAINNET_PROGRAM_ID || '',
    },
    devnet: {
        cluster: 'devnet' as ClusterType,
        endpoint: 'https://api.devnet.solana.com',
        programId: process.env.DEVNET_PROGRAM_ID || '',
    },
    localnet: {
        cluster: 'localnet' as ClusterType,
        endpoint: 'http://localhost:8899',
        programId: process.env.LOCAL_PROGRAM_ID || '',
    },
} as const;

export const DEFAULT_NETWORK = NETWORKS.devnet;

export const CONSTANTS = {
    CLINIC_ACCOUNT_SIZE: 1000,
    RECORD_ACCOUNT_SIZE: 1000,
    MAX_NAME_LENGTH: 50,
    MAX_DIAGNOSIS_LENGTH: 1000,
    MAX_TREATMENT_PLAN_LENGTH: 1000,
} as const;

export const ERROR_MESSAGES = {
    INVALID_NETWORK: 'Invalid network specified',
    INVALID_PROGRAM_ID: 'Program ID not configured for network',
    UNAUTHORIZED: 'Unauthorized access',
    INSUFFICIENT_FUNDS: 'Insufficient funds for transaction',
    CLINIC_NOT_FOUND: 'Clinic not found',
    RECORD_NOT_FOUND: 'Dental record not found',
} as const;