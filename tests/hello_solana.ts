import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloSolana } from "../target/types/hello_solana";
import { Keypair } from '@solana/web3.js';

describe("hello_solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HelloSolana as Program<HelloSolana>;
  const payer = provider.wallet as anchor.Wallet;
  console.log("payer: ", payer.publicKey);

  it("Is initialized!", async () => {
    const tx = await program.methods.initialized().rpc();
    console.log("Your transaction signature", tx);
  });

  // This price feed update has a lower verification level than the one requested.
  it("Fetch price from Pythe", async () => {
    const priceUpdaterKeypair = new Keypair();

    const feedId = "0x097d687437374051c75160d648800f021086bc8edf469f11284491fda8192315";
    const tx = await program.methods.updatePrice(feedId).accounts({
        payer: payer.publicKey,
        priceUpdater: priceUpdaterKeypair.publicKey,
    })
    .signers([priceUpdaterKeypair])
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
