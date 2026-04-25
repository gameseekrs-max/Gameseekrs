On-chain Anchor program for **Vault Runner** (Solana Mobile arcade; PDAs are per this program id).

- Reach Level 3 → FREE Legend SBT (first 1000 mints)
- SKR claim for Legend holders

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
| **This repo** (`Gameseekrs`) | **Public** | `gameseekrs-max/Gameseekrs` | Anchor program, **PRIVACY.md**, **LICENSE.md**, compliance links for stores and forms. |
| **Vault Runner app** (React Native) | **Private** | e.g. `gameseekrs-max/SeekrsRampageApp` | Rename the GitHub repo to match **Vault Runner** branding when convenient; update local `git remote` and any CI. |

There is **no** standalone GameSeekrs marketing website; official social presence is **[@gameseekrs on X](https://x.com/gameseekrs)**. Legal and program transparency use **this public Git** (links above).

---

## Not this project

**Optima Sanitas** / **`sanitas_seeker`** / **`OptimaSanitas-reference-*.zip`** are a **different** codebase and product. They use **different** program IDs and keys. **Do not** copy `PROGRAM_IDS.md`, deployer keys, or compliance text from that zip into Vault Runner or this repo.

---

## Legal files in this repo

- **[PRIVACY.md](./PRIVACY.md)** — Vault Runner / GameSeekrs beta privacy policy (link this URL in app stores, Solana Mobile forms, etc.).
- **[LICENSE.md](./LICENSE.md)** — MIT license for the Anchor program in this repository.
