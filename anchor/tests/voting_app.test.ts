import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { VotingApp } from '../target/types/voting_app'
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";

describe('APP', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.VotingApp as Program<VotingApp>;
  const pollId = new anchor.BN(1);
  const description = "Whats my age";
  const pollStart = new anchor.BN(Date.now() / 1000);
  const pollEnd = new anchor.BN(Date.now() / 1000 + 86400);
  let pollPDA: PublicKey;

  async function confirm(signature: string): Promise<string> {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  }

  async function log(signature: string): Promise<string> {
    console.log(
      ` Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  }

  it('Intialized The poll', async () => {
    pollPDA = anchor.web3.PublicKey.findProgramAddressSync(
      [pollId.toArrayLike(Buffer, "le", 8)],
      program.programId
    )[0];
    await program.methods
      .initializePoll(pollId, description, pollStart, pollEnd)
      .accountsStrict({
        signer: provider.wallet.publicKey,
        poll: pollPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("Initializes a candidate", async () => {
    const pollId = new anchor.BN(1);
    const candidateName = "Rust";
    const [pollPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [pollId.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    const [candidatePDA] = await anchor.web3.PublicKey.findProgramAddress(
      [
        pollId.toArrayLike(Buffer, "le", 8),
        Buffer.from(candidateName)
      ],
      program.programId
    );
    await program.methods
      .initializeCandidate(candidateName, pollId)
      .accountsPartial({
        signer: provider.wallet.publicKey,
        poll: pollPDA,
        candidateAccount: candidatePDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .then(confirm)
      .then(log);
  });
});
