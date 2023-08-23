# examples

Goal of this repository is to show the different possible interactions with UXD Protocol

## Solana UXD Program

### Mint/Redeem CPI

This example aims at demonstrating how to mint and redeem UXD using cross program invocation on the solana blockchain.
Mint and Redeems are ususally done using the mint/redeem program instructions, these use the router under the hood.
These instruction do use a lot of accounts and make composition pretty difficult, as such this examples will use the
instructions minting and redeeming specifically on the Identity Depository.

#### To build

`anchor build`
