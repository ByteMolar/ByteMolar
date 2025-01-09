import { 
    Connection, 
    Keypair, 
    sendAndConfirmTransaction, 
    SystemProgram, 
    Transaction,
    TransactionInstruction,
    PublicKey,
    LAMPORTS_PER_SOL
} from '@solana/web3.js';
import fs from 'fs';
import path from 'path';

async function deployProgram() {
    // Connect to cluster
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    
    // Read keypair file
    const deployerKeypair = Keypair.generate();
    
    // Fund the deployer account
    const airdropSignature = await connection.requestAirdrop(
        deployerKeypair.publicKey,
        LAMPORTS_PER_SOL * 2
    );
    await connection.confirmTransaction(airdropSignature);
    
    // Read the program binary
    const programPath = path.join(__dirname, '../../dist/program/bytemolar.so');
    const programBinary = fs.readFileSync(programPath);
    
    console.log('Deploying ByteMolar program...');
    
    // Deploy the program
    const program = await deployBinaryToChain(
        connection,
        deployerKeypair,
        programBinary
    );
    
    console.log(`Program deployed successfully!`);
    console.log(`Program ID: ${program.publicKey.toString()}`);
    
    // Save program ID for future reference
    const configPath = path.join(__dirname, '../program-id.json');
    fs.writeFileSync(
        configPath,
        JSON.stringify({ programId: program.publicKey.toString() })
    );
}

async function deployBinaryToChain(
    connection: Connection,
    payer: Keypair,
    programBinary: Buffer
): Promise<{ publicKey: PublicKey }> {
    const program = Keypair.generate();
    
    // Calculate deployment cost
    const space = programBinary.length;
    const lamports = await connection.getMinimumBalanceForRentExemption(space);
    
    // Create deployment transaction
    const transaction = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: program.publicKey,
            lamports,
            space,
            programId: SystemProgram.programId,
        })
    );
    
    // Deploy
    await sendAndConfirmTransaction(
        connection,
        transaction,
        [payer, program]
    );
    
    return { publicKey: program.publicKey };
}

deployProgram().catch(console.error);