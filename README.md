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

| Program               | Address                                                                                                                          |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| UXD Program           | [UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr](https://explorer.solana.com/address/UXD8m9cvwk4RcSxnX2HZ9VudQCEeDH6fRnB4CAP57Dr)   |
| Authority (DAO)       | [CzZySsi1dRHMitTtNe2P12w3ja2XmfcGgqJBS8ytBhhY](https://explorer.solana.com/address/CzZySsi1dRHMitTtNe2P12w3ja2XmfcGgqJBS8ytBhhY) |
| Redeemable mint (UXD) | [7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT](https://explorer.solana.com/address/7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT) |
| Controller            | [3tbJcXAWQkFVN26rZPtwkFNvC24sPT35fDxG4M7irLQW](https://explorer.solana.com/address/3tbJcXAWQkFVN26rZPtwkFNvC24sPT35fDxG4M7irLQW) |
| Identity Depository   | [BgkHf7mAtNwtnu2uCJqSJWbFdiXCoMBpNZmgVJJmsGLW](https://explorer.solana.com/address/BgkHf7mAtNwtnu2uCJqSJWbFdiXCoMBpNZmgVJJmsGLW) |
| Credix Depository     | [AGqtEsmCnzQNbSQzM6qmTZ4M5nhJ8WP8CbdNh6eQBuWF](https://explorer.solana.com/address/AGqtEsmCnzQNbSQzM6qmTZ4M5nhJ8WP8CbdNh6eQBuWF) |
| Mercurial Depository  | [4gMkg5iMaYApKQEJ5MDQCVuZ5HZ8Q5GvKwz2sJxRGwyb](https://explorer.solana.com/address/4gMkg5iMaYApKQEJ5MDQCVuZ5HZ8Q5GvKwz2sJxRGwyb) |

## Notable **Devnet** accounts

| Program               | Address      |
| --------------------- | ------------ |
| UXD Program           | [TODO](TODO) |
| Redeemable mint (UXD) | [TODO](TODO) |
| Controller            | [TODO](TODO) |
| Identity Depository   | [TODO](TODO) |

## Lexique

| Word       | Description                                        |
| ---------- | -------------------------------------------------- |
| Redeemable | Represent the token to be redeemed. The UXD token. |
| Collateral | Token provided in exchange of Redeemable.          |
