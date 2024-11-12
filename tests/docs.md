*  list the transactions from an address  
```ts
let web3 = require('@solana/web3.js');

const solanaConnection = new web3.Connection(web3.clusterApiUrl("mainnet-beta"));

const getTransactions = async(address,limit) => {
  const pubKey = new web3.PublicKey(address);
  let transactionList = await solanaConnection.getSignaturesForAddress(pubKey, {limit: limit});
  let signatureList = transactionList.map(transaction => transaction.signature);

  console.log(signatureList);

  for await (const sig of signatureList) {
    console.log(await solanaConnection.getParsedTransaction(sig, {maxSupportedTransactionVersion: 0}));
  }
}

let myAddress = "enter and address here";

getTransactions(myAddress, 3);

```

```
Solana Bytecode Format (SBF)
```

```
Solana account rent When allocating storage space, the payer must pay a certain number of SOL per byte allocated.
Solana calls this the “rent”. 
Nowadays, all accounts are required to be rent-exempt; you cannot pay less than 2 years of rent.
Although rent is computed on a “per byte” basis, accounts with zero data are not free; Solana still has to index them and store metadata about them.

When accounts are initialized, the amount of rent needed is computed in the background; you don’t need to calculate the rent explicitly.
```

```
An owner of an account is a program. An authority is a wallet. 
All accounts in Solana have the following fields, which are mostly self-explanatory:

Public Key
lamport balance
owner
executable (a boolean flag)
rent_epoch (can be ignored for rent-exempt accounts)
data
```

```
The Account type will check that the owner of the account being loaded is actually owned by the program. 

UncheckedAccount is an alias for AccountInfo. This does not check for ownership, so care must be taken as it will accept arbitrary accounts.

This type will check that the Signer account signed the transaction; it checks that the signature matches the public key of the account.

Because a signer is also an account, you can read the Signer’s balance or data (if any) stored in the account, though it’s primary purpose is to validate signatures.

a program, and you may issue to it a cross program invocation.

```





