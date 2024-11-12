import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Rpg } from "../target/types/rpg";
import { Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from '@solana/web3.js';
import { assert } from 'chai';

describe("Rpg", () => {
  let provider = null;
  let program = null;
  let payer = null;

  before(async () => {
    provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
  
    program = anchor.workspace.Rpg as Program<Rpg>;
    payer = provider.wallet as anchor.Wallet;

    console.log("payer     : ", payer.publicKey.toString());
    console.log("program id: ", program.programId.toString());  
  });

  it("Is initialized!", async () => {
    await program.methods.initialize().rpc();
  });


})