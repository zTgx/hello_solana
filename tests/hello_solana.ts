import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloSolana } from "../target/types/hello_solana";
import { Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from '@solana/web3.js';
import { assert } from 'chai';

describe("hello_solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HelloSolana as Program<HelloSolana>;
  const payer = provider.wallet as anchor.Wallet;
  console.log("payer     : ", payer.publicKey.toString());
  console.log("program id: ", program.programId.toString());

  const addressInfoAccount = new Keypair();
  const counterKeypair = new Keypair();
  const userBKeypair = new Keypair();

  it("Is initialized!", async () => {
    await program.methods.initialize().rpc();
  });

  /*
  // TODO:
  // This price feed update has a lower verification level than the one requested.
  // it("Fetch price from Pythe", async () => {
  //   const priceUpdaterKeypair = new Keypair();

  //   const tx = await program.methods.priceUpdate().accounts({
  //       payer: payer.publicKey,
  //       priceUpdater: priceUpdaterKeypair.publicKey,
  //       clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
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

  it('Initialize Counter', async () => {
    await program.methods
      .initializeCounter(new anchor.BN(2))
      .accounts({
        counter: counterKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([counterKeypair])
      .rpc();

    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);

    assert(currentCount.count.toNumber() === 0, 'Expected initialized count to be 0');
    assert(currentCount.maxCount.toNumber() === 2, 'Expected initialized max_count to be 2');
  });

  it('Increment Counter', async () => {
    await program.methods.increment().accounts({ counter: counterKeypair.publicKey }).rpc();

    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);

    assert(currentCount.count.toNumber() === 1, 'Expected  count to be 1');
  });

  it('Increment Counter Again', async () => {
    await program.methods.increment().accounts({ counter: counterKeypair.publicKey }).rpc();

    const currentCount = await program.account.counter.fetch(counterKeypair.publicKey);

    assert(currentCount.count.toNumber() === 2, 'Expected  count to be 2');
  });

  it("Should handle counter overflow", async () => {
    // Attempt to increment the counter, expecting an overflow error
    try {
      await program.methods.increment().accounts({ counter: counterKeypair.publicKey }).rpc();
      assert.fail("Expected an overflow error but did not receive one.");
    } catch (error) {
      assert.include(error.message, "Overflow", "Expected overflow error message");
    }
  });

  it('Should always throw an error', async () => {
    try {
      await program.methods.errorExample().rpc();
      assert.fail("Expected an error but did not receive one.");
    } catch (error) {
      assert.include(error.message, "Always", "Expected 'Always' error message");
    }
  });

  it('Set and Get Favorites', async () => {
    const number = 42;
    const color = "Blue";
    const hobbies = ["Reading", "Hiking", "Coding"];

    const [favoritesKeypair, avoritesPdaAndBump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('favorites'), payer.publicKey.toBuffer()], program.programId);

    const tx = await program.methods
      .setFavorites(new anchor.BN(number), color, hobbies)
      .accounts({
        favorites: favoritesKeypair,
        payer: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();

    console.log("tx: ", tx);
    const favoritesAccount = await program.account.favorites.fetch(favoritesKeypair);

    assert(favoritesAccount.number.toNumber() === number, `Expected number to be ${number}`);
    assert(favoritesAccount.color === color, `Expected color to be ${color}`);
    assert.deepEqual(favoritesAccount.hobbies, hobbies, `Expected hobbies to be ${hobbies}`);
  });

  it('Update Favorites', async () => {
    const updatedNumber = 43;
    const updatedColor = "Green";
    const updatedHobbies = ["Reading", "Hiking", "Coding", "Photography"];

    const [favoritesKeypair, avoritesPdaAndBump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('favorites'), payer.publicKey.toBuffer()], program.programId);

    const tx = await program.methods
      .setFavorites(new anchor.BN(updatedNumber), updatedColor, updatedHobbies)
      .accounts({
        favorites: favoritesKeypair,
        payer: payer.publicKey,
      })
      .signers([payer.payer])
      .rpc();
    console.log("tx: ", tx);

    const updatedFavoritesAccount = await program.account.favorites.fetch(favoritesKeypair);

    assert(updatedFavoritesAccount.number.toNumber() === updatedNumber, `Expected number to be ${updatedNumber}`);
    assert(updatedFavoritesAccount.color === updatedColor, `Expected color to be ${updatedColor}`);
    assert.deepEqual(updatedFavoritesAccount.hobbies, updatedHobbies, `Expected hobbies to be ${updatedHobbies}`);
  });

  // We'll create this ahead of time.
  // Our program will try to modify it.
  const accountToChange = new Keypair();
  // Our program will create this.
  const accountToCreate = new Keypair();

  it('Create an account owned by our program', async () => {
    const instruction = SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(0),
      space: 0,
      programId: program.programId, // Our program
    });

    const transaction = new Transaction().add(instruction);

    await sendAndConfirmTransaction(provider.connection, transaction, [payer.payer, accountToChange]);
  });

  it('Check accounts', async () => {
    await program.methods
      .checkAccounts()
      .accounts({
        payer: payer.publicKey,
        accountToCreate: accountToCreate.publicKey,
        accountToChange: accountToChange.publicKey,
      })
      .rpc();
  });
  */

  // Derive the PDA for the user's account.
  const [userAccountAddress] = PublicKey.findProgramAddressSync([Buffer.from('USER'), payer.publicKey.toBuffer()], program.programId);

  it('Create Account', async () => {
    await program.methods
      .createUser('John Doe')
      .accounts({
        user: payer.publicKey,
        userAccount: userAccountAddress,
      })
      .rpc();

    // Fetch the account data
    const userAccount = await program.account.userState.fetch(userAccountAddress);
    assert.equal(userAccount.name, 'John Doe');
    assert.equal(userAccount.user.toBase58(), payer.publicKey.toBase58());
  });

  it('Close Account', async () => {
    await program.methods
      .closeUser()
      .accounts({
        user: payer.publicKey,
        userAccount: userAccountAddress,
      })
      .rpc();

    // The account should no longer exist, returning null.
    const userAccount = await program.account.userState.fetchNullable(userAccountAddress);
    assert.equal(userAccount, null);
  });

  it("System vars!", async () => {
    const tx = await program.methods
      .systemVars()
      .accounts({
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
      })
      .rpc();

    console.log("Transaction hash got:", tx);
  });

  it("Emit evnets", async () => {
    const listenerMyEvent = program.addEventListener('MyEvent', (event, slot) => {
      console.log(`slot ${slot} event value ${event.value}`);
    });

    await program.methods.emitEvents().rpc();

    // This line is only for test purposes to ensure the event
    // listener has time to listen to event.
    await new Promise((resolve) => setTimeout(resolve, 5000));

    program.removeEventListener(listenerMyEvent);
  });

  it("Is called by the owner", async () => {
    // Add your test here.
    const tx = await program.methods
      .onlyOwner()
      .accounts({
        signerAccount: payer.publicKey,
      })
      .rpc();

    console.log("Only Owner transaction hash:", tx);
  });

  let attackKeypair = anchor.web3.Keypair.generate();

  it("Is NOT called by the owner", async () => {
    try {
      await program.methods
      .initialize()
      .accounts({
        signerAccount: attackKeypair.publicKey,
      })
      .signers([attackKeypair])
      .rpc();
      
      assert.fail("Expected an owner.");
    } catch (error) {
      assert.include(error.message, "unknown signer", "Not Owner");
    }
  });


});
