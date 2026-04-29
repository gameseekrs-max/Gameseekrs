# seeker-rampage — continuation handoff

**Repo:** Anchor program(s) for the Seeker Rampage / Vault Runner stack.

## SNS / product line

**GameSeekrs** public program + legal docs live in **`gameseekrs-max/Gameseekrs`**. Same SNS / HTTPS identity as the apps: **`gameseekrs.sol`** → **`https://gameseekrs.sol.site`** (see **`../VaultRunner/docs/continuation.md`** and **`~/Groking/Games/continuation.md`**). Not Optima (**`optimasanitas.sol`**).

## Long-form context

- **App, wallet, store, Flashlight, deploy:** use **`../VaultRunner/docs/continuation.md`** in the **Games** monorepo (or the **VaultRunner** clone under `~/Groking/Games/VaultRunner` on this machine).

## This tree (quick map)

| Path | Notes |
|------|--------|
| `programs/` | Solana program source |
| `Anchor.toml`, `Cargo.toml` | Build / cluster / program id |
| `docs/` | Program / listing-adjacent docs |
| `README.md` | Anchor / deploy commands |

## Portable snapshot (GitHub-oriented export)

**2026-04-27:** A zip of **tracked + untracked non-ignored** files (respects `.gitignore`, e.g. excludes `node_modules`) lives at:

`~/Groking/seeker-rampage-github-export-20260427.zip`

Purpose: offline backup or review before **git add / commit / push**. The pack script also **drops** `node_modules`, **`target/`**, Android/iOS build dirs, etc. — see **`~/Groking/Games/continuation.md`**.
