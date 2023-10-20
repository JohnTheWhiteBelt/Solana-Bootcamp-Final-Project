# Freezable NFT

Based on the tutorial nft.yaml, added a field `freezed` to `GemMetadata`, and a method `freeze`. Once the NFT is freezed, `transfer` & `burn` can't be performed on it.

## prerequisite
1. Install Rust, Rustup and Cargo
2. Install Solana Tool Suite 
3. Create Wallet
```
solana-keygen new 
```
4. Set config to devnet and get SOL for gas fee
```
solana config set --url devnet
solana airdrop 1 
```
* at least 3 SOL needed to deploy and interact with the program.

## How to run

```
cd program
cargo build-sbf
solana program deploy target/deploy/nft.so
```
Write down the ProgramId from the output of the deployment
```
cc ../program_client
yarn install
npx ts-node app.ts <YOUR_PROGRAM_ID>
```