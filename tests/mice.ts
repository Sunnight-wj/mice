import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection, SYSVAR_RENT_PUBKEY, SystemProgram, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import { Mice } from "../target/types/mice";
import { getAssociatedTokenAddressSync, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";


import * as dotenv from "dotenv";
dotenv.config();

const provider = anchor.AnchorProvider.env();
const connection = provider.connection;
  anchor.setProvider(provider);
  const payer = provider.wallet as Wallet;
  const program = anchor.workspace.Mice as Program<Mice>;
  
const mintAccount = new PublicKey("DuDdvHkinLHsHGmN8havzZAtEoR593NcQGPRpHBn2j3n");
const spenderAuthourity = PublicKey.findProgramAddressSync(
  [
    mintAccount.toBuffer(),
    Buffer.from("authority")
  ],
  anchor.workspace.Mice.programId
)[0];

const ownerTokenAccount = getAssociatedTokenAddressSync(
  mintAccount,
  payer.publicKey
)

describe("mice", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Approve!", async () => {

    const amount = new anchor.BN("5000")
    const tx = await program.methods
      .approveNft(amount)
      .accounts({
        owner: payer.publicKey,
        mintAccount,
        spenderAuthourity,
        ownerTokenAccount,
      })
      .signers([payer.payer])
      .rpc();
    console.log(" transaction signature", tx);
  });

  it("Mint!", async () => {
    const user = new Keypair();
    const recipient = new Keypair();
    console.log(`user: ${user.publicKey}, recipient: ${recipient.publicKey}`);

    const userTokenAccount = getAssociatedTokenAddressSync(
      mintAccount,
      user.publicKey
    );

    const transferInstruction = SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: user.publicKey,
      lamports: 300000000,
    })
    const transaction = new Transaction();
    transaction.add(transferInstruction);
    const sx = await sendAndConfirmTransaction(
      connection,
      transaction,
      [payer.payer]
    )
    console.log(`transfer SOL success`)

    const tx = await program.methods
      .mintNft()
      .accounts({
        user: user.publicKey,
        from: payer.publicKey,
        recipient: recipient.publicKey,
        mintAccount,
        userTokenAccount,
        fromTokenAccount: ownerTokenAccount,
        spenderAuthourity,
      })
      .signers([user])
      .rpc();
  });

  it("Transfer!", async () => {
    const recipient = new Keypair();
    console.log(`Transfer recipient: ${recipient.publicKey}`)
    const tx = await program.methods
      .transferSol()
      .accounts({
        user: payer.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();
  });
});
