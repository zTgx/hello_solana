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

  const addressInfoAccount = new Keypair();

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  // TODO:
  // This price feed update has a lower verification level than the one requested.
  // it("Fetch price from Pythe", async () => {
  //   const priceUpdaterKeypair = new Keypair();

  //   const feedId = "0x097d687437374051c75160d648800f021086bc8edf469f11284491fda8192315";
  //   const tx = await program.methods.priceUpdate(feedId).accounts({
  //       payer: payer.publicKey,
  //       priceUpdater: priceUpdaterKeypair.publicKey,
  //   })
  //   .signers([priceUpdaterKeypair])
  //   .rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it('Create the address info account', async () => {
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

    // Instruction Ix data
    const addressInfo = {
      name: 'Joe C',
      houseNumber: 136,
      street: 'Mile High Dr.',
      city: 'Solana Beach',
    };

    const tx = await program.methods
      .createAddressInfo(addressInfo.name, addressInfo.houseNumber, addressInfo.street, addressInfo.city)
      .accounts({
        addressInfo: addressInfoAccount.publicKey,
        payer: payer.publicKey,
      })
      .signers([addressInfoAccount])
      .rpc();

      console.log("Your transaction signature", tx);
  });

  it("Read the new account's data", async () => {
    const addressInfo = await program.account.addressInfo.fetch(addressInfoAccount.publicKey);
    console.log(`Name     : ${addressInfo.name}`);
    console.log(`House Num: ${addressInfo.houseNumber}`);
    console.log(`Street   : ${addressInfo.street}`);
    console.log(`City     : ${addressInfo.city}`);
  });






});
