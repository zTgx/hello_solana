Program Derived Addresses (PDAs) provide developers on Solana with two main use cases:

Deterministic Account Addresses: PDAs provide a mechanism to deterministically derive an address using a combination of optional "seeds" (predefined inputs) and a specific program ID.
Enable Program Signing: The Solana runtime enables programs to "sign" for PDAs which are derived from its program ID.

Key Points #
PDAs are addresses derived deterministically using a combination of user-defined seeds, a bump seed, and a program's ID.

PDAs are addresses that fall off the Ed25519 curve and have no corresponding private key.

Solana programs can programmatically "sign" for PDAs that are derived using its program ID.

Deriving a PDA does not automatically create an on-chain account.

An account using a PDA as its address must be explicitly created through a dedicated instruction within a Solana program.

A PDA is a point that is intentionally derived to fall off the Ed25519 curve using a predefined set of inputs. A point that is not on the Ed25519 curve does not have a valid corresponding private key and cannot be used for cryptographic operations (signing).

The derivation of a PDA requires 3 inputs.

Optional seeds: Predefined inputs (e.g. string, number, other account addresses) used to derive a PDA. These inputs are converted to a buffer of bytes.
Bump seed: An additional input (with a value between 255-0) that is used to guarantee that a valid PDA (off curve) is generated. This bump seed (starting with 255) is appended to the optional seeds when generating a PDA to "bump" the point off the Ed25519 curve. The bump seed is sometimes referred to as a "nonce".
Program ID: The address of the program the PDA is derived from. This is also the program that can "sign" on behalf of the PDA

