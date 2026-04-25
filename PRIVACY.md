# Privacy Policy — GameSeekrs: Vault Runner (Beta)

**Last updated:** April 25, 2026

## Who we are

**GameSeekrs** publishes **Vault Runner**, a Solana Mobile game. We do **not** operate a standalone website. Official updates and contact are on **X**: [@gameseekrs](https://x.com/gameseekrs).

This policy covers the **Vault Runner** mobile app (React Native client; **not** open-sourced) and its use of the **Vault Runner** on-chain program on Solana (**open-source** in this repository).

## On-chain program (transparent)

- **Devnet program id:** `J1K3v6h1gWDbMqTZCgCzhptHLphCmxqh11fJEMZhcTJA`
- **Beta:** gameplay and rewards logic are exercised on **Solana Devnet** until you are told otherwise.
- **Anchor source:** same repo as this file — see **`README.md`** and **`LICENSE.md`**.

## What the app collects

- **Wallet connection:** When you use Solana Mobile Wallet Adapter (or compatible wallets), your **wallet public key** is used to sign transactions and derive on-chain accounts. We do **not** receive your seed phrase or private key; signing happens in your wallet.
- **On-chain data:** High scores, gameplay transactions, optional SBT mints, and token claims are recorded **on Solana** as public blockchain data. Anyone can observe those transactions on-chain.
- **Location / sensors:** If enabled in a build, device features are used only as implemented in that build’s code (see app permissions on the store listing). There is **no** in-app ad analytics SDK in the reference client described by this policy; if that changes, this document will be updated on GitHub.

## What we do not do

- No sale of personal data.
- No account system beyond your self-custodial wallet.
- No cookies (this is a mobile app, not a website).

## Children

Vault Runner is not directed at children under 13 (or the age required in your jurisdiction). On-chain wallet use is intended for users who can lawfully agree to wallet terms.

## Third parties

- **Solana RPC:** Your app sends transactions and reads chain state via Solana RPC endpoints configured in the app (e.g. public devnet/mainnet endpoints). Those providers see IP and request metadata under **their** policies.
- **X (Twitter):** If you open our X profile from a link inside the app, X’s privacy policy applies there.

## Changes

We may update this policy by committing a new version to this **public** GitHub repository. The “Last updated” date will change at the top of this file.

## Contact

- **X:** [@gameseekrs](https://x.com/gameseekrs)
- **Program / legal docs:** this repo — [Privacy (this file)](https://github.com/gameseekrs-max/Gameseekrs/blob/main/PRIVACY.md) (update path if your published branch or folder layout differs).

---

*Optima Sanitas / `sanitas_seeker` materials in separate reference archives are **not** this product and use **different** program IDs. Do not mix compliance or keys between those projects and Vault Runner.*
