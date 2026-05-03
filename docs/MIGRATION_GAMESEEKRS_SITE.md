# Migration: public hub → `gameseekrs-site`

The GameSeekrs **landing + legal HTML** now has a **dedicated** public repo, same *deployment* pattern as Optima’s **`optimasanitas-site`**, but **its own** layout and stylesheets (**`assets/hub-site.css`**, **`assets/legal.css`**) — do not visually clone the Optima hub.

**`https://github.com/gameseekrs-max/gameseekrs-site`**

## Why

- **Gameseekrs** (this repo) stays focused on the **Anchor program** + markdown sources.
- **gameseekrs-site** holds **`index.html`**, per-app folders **`vault-runner/`** and **`flashlight/`**, and shared **`assets/legal.css`** + **`assets/hub-site.css`** at the site root — better for GitHub Pages **`/`**, IPFS, and Firebase.

**Continuation / agent handoffs:** This **`Gameseekrs`** repo is **public** — root **`CONTINUATION.md`** is a **stub** only. Full narrative lives in **private** **`gameseekrs-max/VaultRunner`** → **`docs/continuation.md`** (see section *Continuation files — private repos only* there).

## New path map (after Pages enabled on `gameseekrs-site`)

| Old (this repo `docs/legal/`) | New (`gameseekrs-site`) |
|--------------------------------|-------------------------|
| `…/legal/vault-privacy.html` | `…/vault-runner/privacy.html` |
| `…/legal/vault-terms.html` | `…/vault-runner/terms.html` |
| `…/legal/flashlight-privacy.html` | `…/flashlight/privacy.html` |
| `…/legal/flashlight-terms.html` | `…/flashlight/terms.html` |
| `…/legal/license.html` | `…/vault-runner/license.html` or `…/flashlight/license.html` (same MIT text) |

Base URL example:

`https://gameseekrs-max.github.io/gameseekrs-site/`

## What you need to update

1. **VaultRunner** and **GSkrsFlashlight** — `src/config/publicLinks.ts`: set **`GAMESEEKRS_LEGAL_PAGES_BASE`** (or equivalent) to the new base URL + paths above.
2. **Store / MWA** listings — any hard-coded **Gameseekrs** `docs/legal` URLs → new paths.
3. **SNS / Sol.site** — when ready, point **`gameseekrs.sol.site`** at **`gameseekrs-max.github.io`** for **`gameseekrs-site`** (not **`Gameseekrs`**) per [website configuration](https://docs.sns.id/collection/sns-v2/sol.site/website-configuration.md). Only **one** GitHub Pages site should own that hostname.
4. **This repo** — `docs/index.html` + `docs/legal/*.html` can stay temporarily for redirects or be trimmed once traffic moves.

## Markdown source of truth

Legal copy still originates from **`PRIVACY.md`**, **`docs/PRIVACY_GSKRS_FLASHLIGHT.md`**, **`docs/TERMS_*.md`**, **`LICENSE.md`**. When you edit those, sync the HTML in **gameseekrs-site**.
