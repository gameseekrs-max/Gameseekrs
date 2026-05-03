On-chain Anchor program for **Vault Runner** — a **GameSeekrs** arcade title **for [Solana Mobile Seeker](https://solanamobile.com/seeker)** (Android). PDAs are per this program id.

- Reach Level 3 → Legend SBT offer (eligibility in published app / terms when a mainnet program is announced)
- Optional on-chain features for qualified accounts follow the deployed program; **not** a public promise of any SKR distribution until you publish that program and policies

**Open-source:** Only the Anchor smart contract (for full on-chain transparency).  
The **React Native** Vault Runner client is developed separately and is **not** required to be open-source.

**Vault Runner program (this repo, not legacy zip IDs):**  
Devnet program id: `J1K3v6h1gWDbMqTZCgCzhptHLphCmxqh11fJEMZhcTJA`  
Deploy keypair: `target/deploy/seeker_rampage-keypair.json` (gitignored with `target/`).  
Provider wallet: `vault-runner-authority.json` (gitignored) — pubkey must match `DEPLOYER_AUTHORITY` in `lib.rs`.

**GameSeekrs** is the brand for Seeker **games and utilities** — **Vault Runner** (arcade) and **GSkrs Flashlight** (flashlight and tools). Official hub and styled legal HTML: **`https://gameseekrs.sol.site`** ([gameseekrs-max/gameseekrs-site](https://github.com/gameseekrs-max/gameseekrs-site)). Social: **[@gameseekrs on X](https://x.com/gameseekrs)**.

This repository holds the **Anchor** program (open for review), **markdown** privacy/terms sources, and links stores can use for compliance.

---

## Not this project

Third-party reference archives and **other teams’** on-chain programs use **different** IDs and keys. **Do not** copy their `PROGRAM_IDS.md`, deployer keys, or legal text into Vault Runner or this repo.

---

## Legal files in this repo

- **[PRIVACY.md](./PRIVACY.md)** — Vault Runner / GameSeekrs beta privacy policy (link this URL for **Vault Runner** store listings and forms).
- **[docs/PRIVACY_GSKRS_FLASHLIGHT.md](./docs/PRIVACY_GSKRS_FLASHLIGHT.md)** — **GSkrs Flashlight** privacy policy (link this URL for the **Flashlight** dApp Store listing; complements `PRIVACY.md` above).
- **[LICENSE.md](./LICENSE.md)** — MIT license for the Anchor program in this repository.
