# `gameseekrs.sol` → Sol.site + GitHub Pages

**Product line:** GameSeekrs (**gameseekrs-max**) — games and utilities on Seeker; **Vault Runner** and **GSkrs Flashlight** ship today; the same **`gameseekrs.sol.site`** hub covers future titles — extend **`gameseekrs-site`** (`index.html` + `vault-runner/` / `flashlight/` or new folders). Legacy **`docs/`** pages optional during migration. Not Optima Sanitas.

**Site styling:** **`gameseekrs-site`** must keep its **own** layout and CSS versus **`OptimaSanitas/optimasanitas-site`** (see private **`gameseekrs-max/VaultRunner`** `docs/continuation.md`, section *2026-05-03 — gameseekrs-site visual identity*). Same URL shape ≠ same visual theme.

## URLs

| Purpose | Value |
|---------|--------|
| **SNS name** | `gameseekrs.sol` |
| **HTTPS (MWA `identity.uri`, reviewers)** | `https://gameseekrs.sol.site` |
| **Code** | `GAMESEEKRS_MWA_IDENTITY_URI` in **`VaultRunner`** / **`GSkrsFlashlight`** `src/config/publicLinks.ts` |
| **Legal HTML + landing (preferred)** | **`gameseekrs-max/gameseekrs-site`**: root **`index.html`**, **`vault-runner/*.html`**, **`flashlight/*.html`** — migrate `GAMESEEKRS_LEGAL_PAGES_BASE` here (see **`docs/MIGRATION_GAMESEEKRS_SITE.md`**). |
| **Legal HTML (legacy)** | **`gameseekrs-max/Gameseekrs`**: **`docs/index.html`** + **`docs/legal/*.html`** until migration completes. |
| **`docs/CNAME`** | Contains **`gameseekrs.sol.site`** for GitHub Pages custom domain (push with `seeker-rampage` → **Gameseekrs**). |

## Wiring checklist

1. Register **`gameseekrs.sol`** on [sns.id](https://sns.id).
2. **Sol.site → Configure:** CNAME to **`gameseekrs-max.github.io`** ([SNS docs](https://docs.sns.id/collection/sns-v2/sol.site/website-configuration.md)).
3. **Custom domain (pick one deployment):**
   - **Preferred:** Repo **`gameseekrs-max/gameseekrs-site`** → Pages from **`/`** on **`main`** → **Custom domain** `gameseekrs.sol.site` (add **`CNAME`** file in that repo when ready). Remove or retarget domain on **Gameseekrs** so only one site owns the hostname.
   - **Legacy:** Repo **`gameseekrs-max/Gameseekrs`** → Pages from **`/docs`** → **`docs/CNAME`** already names **`gameseekrs.sol.site`**.
4. Open **`https://gameseekrs.sol.site/`** — should load the active hub (`gameseekrs-site` root or **`Gameseekrs` docs** until you switch).

Until DNS resolves, wallet sheets still target **`https://gameseekrs.sol.site`** — finish SNS + Pages before store submission.
