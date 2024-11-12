```
The system program owns wallets and keypair accounts that haven’t been assigned ownership to a program (initialized).
The BPFLoader owns programs.
A program owns Solana PDAs. It can also own keypair accounts if ownership has been transferred to the program (this is what happens during initialization).

Programs can transfer ownership of owned accounts
```

Although you “own” your wallet in some metaphysical sense, you do not directly have the ability to write data into it or reduce the lamport balance because, from Solana runtime perspective, you are not the owner.

The reason you are able to spend SOL in your wallet is because you possess the private key that generated the address, or public key. When the system program recognizes that you have produced a valid signature for the public key, then it will recognize your request to spend the lamports in the account as legitimate, then spend them according to your instructions.

The reason programs can write to PDAs or keypair accounts that were created outside the program but initialized by the program, is because the program owns them.


https://www.rareskills.io/post/solana-account-owner#:~:text=The%20system%20program%20owns%20wallets,is%20what%20happens%20during%20initialization).