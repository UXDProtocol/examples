## Introduction

This repository contains documentation about UXD Protocol and showcase how to interact with the UXD Program using Cross-program invocation.

## General Accounts Architecture

UXD Program works around the `Controller` account and the `Depositories` accounts.

The `Controller` is an unique top-level account containing high-level information about the protocol, i.e redeemable mint, authority etc.

A `Depository` account represents one strategy of investment.

- Credix Depository: Invest USDC into [Credix finance](https://credix.finance/).
- Mercurial Vault Depository: Invest USDC in [Meteora protocol](https://www.meteora.ag/).
- Identity Depository: Keep USDC as it is.

## Notable Links

| Title                        | Link                                  | Description                                                              |
| ---------------------------- | ------------------------------------- | ------------------------------------------------------------------------ |
| Backoffice                   | https://uxd-backoffice.vercel.app/    | Displays real time onchain infos about UXD Program.                      |
| Governance                   | https://realms.today/dao/UXP          | DAO having authority over the UXD Protocol treasury and the UXD Program. |
| Frontend                     | https://app.uxd.fi/                   | UXD Protocol frontend.                                                   |
| Non-technical Documentation  | https://docs.uxd.fi/uxdprotocol/      | Explains what is UXD about.                                              |
| Medium                       | https://uxdprotocol.medium.com/       | Medium account referencing interesting articles related to UXD.          |
| Official X (Twitter) account | https://twitter.com/UXDProtocol       | Follow for news.                                                         |
| Official Discord account     | https://discord.com/invite/BHfpYmjsBM | Join to discuss UXD related topics.                                      |
| Github                       | https://github.com/uxdprotocol        | Contains open source UXD Protocol code.                                  |

## Notable **Mainnet** accounts

| Program                | Address                                                                                                                          |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| Authority (DAO)        | [CzZySsi1dRHMitTtNe2P12w3ja2XmfcGgqJBS8ytBhhY](https://explorer.solana.com/address/CzZySsi1dRHMitTtNe2P12w3ja2XmfcGgqJBS8ytBhhY) |
| UXD Program            | [UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr](https://explorer.solana.com/address/UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr)   |
| Collateral mint (USDC) | [EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v](https://explorer.solana.com/address/EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v) |
| Redeemable mint (UXD)  | [7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT](https://explorer.solana.com/address/7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT) |
| Controller             | [3tbJcXAWQkFVN26rZPtwkFNvC24sPT35fDxG4M7irLQW](https://explorer.solana.com/address/3tbJcXAWQkFVN26rZPtwkFNvC24sPT35fDxG4M7irLQW) |
| Identity Depository    | [BgkHf7mAtNwtnu2uCJqSJWbFdiXCoMBpNZmgVJJmsGLW](https://explorer.solana.com/address/BgkHf7mAtNwtnu2uCJqSJWbFdiXCoMBpNZmgVJJmsGLW) |
| Mercurial Depository   | [4gMkg5iMaYApKQEJ5MDQCVuZ5HZ8Q5GvKwz2sJxRGwyb](https://explorer.solana.com/address/4gMkg5iMaYApKQEJ5MDQCVuZ5HZ8Q5GvKwz2sJxRGwyb) |
| Credix Depository      | [AGqtEsmCnzQNbSQzM6qmTZ4M5nhJ8WP8CbdNh6eQBuWF](https://explorer.solana.com/address/AGqtEsmCnzQNbSQzM6qmTZ4M5nhJ8WP8CbdNh6eQBuWF) |

## Notable **Devnet** accounts

| Program                | Address                                                                                                                                         |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Authority (script key) | [aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC](https://explorer.solana.com/address/aca3VWxwBeu8FTZowJ9hfSKGzntjX68EXh1N9xpE1PC?cluster=devnet)   |
| UXD Program            | [CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu](https://explorer.solana.com/address/CW5VzSk7WC4NPyuNt19VFev9FUHhyk5xxHTj2DUWBexu?cluster=devnet) |
| Collateral mint (USDC) | [Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr](https://explorer.solana.com/address/Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr?cluster=devnet) |
| Redeemable mint (UXD)  | [AgM8ntTipTimvUgjGKwumunfPvLhcjNLQ8F147oZvXhm](https://explorer.solana.com/address/AgM8ntTipTimvUgjGKwumunfPvLhcjNLQ8F147oZvXhm?cluster=devnet) |
| Controller             | [UbgH7eSCxgbr7EWk3LYSA1tVCpX617oefgcgzZu5uvV](https://explorer.solana.com/address/UbgH7eSCxgbr7EWk3LYSA1tVCpX617oefgcgzZu5uvV?cluster=devnet)   |
| Identity Depository    | [AZ8UfkMT6PzzNjauBkwf7XctUVHSRVapzcGqsTFeNs5h](https://explorer.solana.com/address/AZ8UfkMT6PzzNjauBkwf7XctUVHSRVapzcGqsTFeNs5h?cluster=devnet) |
| Mercurial Depository   | [Hpi7FwVY6zUGvbgL2GUPnEmmnU1YfTDZsYPc1oq2ret9](https://explorer.solana.com/address/Hpi7FwVY6zUGvbgL2GUPnEmmnU1YfTDZsYPc1oq2ret9?cluster=devnet) |
| Credix Depository      | [GbT1xUWY1ABi71UjjcUKbHrupYjf8nrwrijt3TjGaK2K](https://explorer.solana.com/address/GbT1xUWY1ABi71UjjcUKbHrupYjf8nrwrijt3TjGaK2K?cluster=devnet) |

Note: to get devnet's USDC, head to the faucet here: https://spl-token-faucet.com/?token-name=USDC-Dev

## Lexique

| Word       | Description                                        |
| ---------- | -------------------------------------------------- |
| Redeemable | Represent the token to be redeemed. The UXD token. |
| Collateral | Token provided in exchange of Redeemable.          |