On-chain Anchor program for **Vault Runner** — a **GameSeekrs** arcade title **for [Solana Mobile Seeker](https://solanamobile.com/seeker)** (Android). PDAs are per this program id.

**Handoff (2026-04-26):** Privacy + README copy updated for **Seeker**; **`main` pushed** to **`gameseekrs-max/Gameseekrs`**. **GSkrs Flashlight** client is **not** this repo; legal for Flashlight is `docs/PRIVACY_GSKRS_FLASHLIGHT.md` (Flashlight’s own git remote **not** created yet on the dev machine used for that pass).

- Reach Level 3 → Legend SBT offer (eligibility in published app / terms when a mainnet program is announced)
- Optional on-chain features for qualified accounts follow the deployed program; **not** a public promise of any SKR distribution until you publish that program and policies

**Open-source:** Only the Anchor smart contract (for full on-chain transparency).  
The **React Native** Vault Runner client is developed separately and is **not** required to be open-source.

**Vault Runner program (this repo, not legacy zip IDs):**  
Devnet program id: `J1K3v6h1gWDbMqTZCgCzhptHLphCmxqh11fJEMZhcTJA`  
Deploy keypair: `target/deploy/seeker_rampage-keypair.json` (gitignored with `target/`).  
Provider wallet: `vault-runner-authority.json` (gitignored) — pubkey must match `DEPLOYER_AUTHORITY` in `lib.rs`.

---

## Repositories (GameSeekrs)

| Piece | Visibility | Typical GitHub remote | Notes |
|--------|------------|----------------------|--------|
| **This repo** (`Gameseekrs`) | **Public** | `gameseekrs-max/Gameseekrs` | Anchor program, **PRIVACY.md**, **LICENSE.md**, **[docs/PRIVACY_GSKRS_FLASHLIGHT.md](./docs/PRIVACY_GSKRS_FLASHLIGHT.md)** (second-app privacy), compliance links for stores and forms. |
| **Vault Runner app** (React Native) | **Private** | e.g. `gameseekrs-max/SeekrsRampageApp` | Rename the GitHub repo to match **Vault Runner** branding when convenient; update local `git remote` and any CI. |
| **GSkrs Flashlight** (React Native) | **Private** (typical) | e.g. `gameseekrs-max/GSkrsFlashlight` | Seeker companion app; listing privacy URL = **`docs/PRIVACY_GSKRS_FLASHLIGHT.md`** in this public repo. |

**Public HTTPS (SNS `gameseekrs.sol` + Sol.site):** **`https://gameseekrs.sol.site`** — same GitHub Pages content as below, once the custom domain is configured. **[@gameseekrs on X](https://x.com/gameseekrs)** is for social updates. Legal and program transparency also use **this public Git** (links above).

---

## Not this project

**Optima Sanitas** / **`sanitas_seeker`** / **`OptimaSanitas-reference-*.zip`** are a **different** codebase and product. They use **different** program IDs and keys. **Do not** copy `PROGRAM_IDS.md`, deployer keys, or compliance text from that zip into Vault Runner or this repo.

---

## Legal files in this repo

- **[PRIVACY.md](./PRIVACY.md)** — Vault Runner / GameSeekrs beta privacy policy (link this URL for **Vault Runner** store listings and forms).
- **[docs/PRIVACY_GSKRS_FLASHLIGHT.md](./docs/PRIVACY_GSKRS_FLASHLIGHT.md)** — **GSkrs Flashlight** privacy policy (link this URL for the **Flashlight** dApp Store listing; complements `PRIVACY.md` above).
- **[LICENSE.md](./LICENSE.md)** — MIT license for the Anchor program in this repository.
