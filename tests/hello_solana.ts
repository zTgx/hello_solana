import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloSolana } from "../target/types/hello_solana";
import { Keypair } from '@solana/web3.js';

describe("hello_solana", () => {
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HelloSolana as Program<HelloSolana>;
  const payer = provider.wallet as anchor.Wallet;
  console.log("payer: ", payer.publicKey);

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const priceUpdaterKeypair = new Keypair();

    const tx = await program.methods.ddd().accounts({
        payer: payer.publicKey,
        priceUpdater: priceUpdaterKeypair.publicKey,
    })
    .signers([priceUpdaterKeypair])
    .rpc();
    console.log("Your transaction signature", tx);
  });


  // it('Say Sample!', async () => {
  //   // Just run Anchor's IDL method to build a transaction!
  //   const priceUpdaterKeypair = new Keypair();
  //   console.log("priceUpdaterKeypair: ", priceUpdaterKeypair.publicKey);

  //   // Check if the account already exists
  //   const accountInfo = await provider.connection.getAccountInfo(priceUpdaterKeypair.publicKey);
    
  //   if (accountInfo === null) {
  //     // Initialize the price_updater account
  //     await program.methods.sample().accounts({
  //       payer: payer.publicKey,
  //       priceUpdater: priceUpdaterKeypair.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId, // Include the system program
  //     })
  //     .signers([priceUpdaterKeypair]) // Sign with both payer and priceUpdaterKeypair
  //     .rpc();
  //   } else {
  //     console.log("Account already exists, skipping initialization.");
  //   }
  // });
});
