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

