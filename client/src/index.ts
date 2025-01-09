import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { ByteMolarClient } from './bytemolar';

export async function initializeByteMolar(
  endpoint: string,
  programId: string
): Promise<ByteMolarClient> {
  const connection = new Connection(endpoint, 'confirmed');
  const programPubkey = new PublicKey(programId);
  
  return new ByteMolarClient(connection, programPubkey);
}

export { ByteMolarClient };
export * from './bytemolar';

// Example usage:
// const client = await initializeByteMolar('https://api.devnet.solana.com', 'your_program_id');