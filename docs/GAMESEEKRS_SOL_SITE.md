# `gameseekrs.sol` → Sol.site + GitHub Pages

**Product line:** GameSeekrs (**gameseekrs-max**) — games and utilities on Seeker; **Vault Runner** and **GSkrs Flashlight** are the shipping titles today; the same **`gameseekrs.sol.site`** hub is meant to cover **future GameSeekrs apps** (extend `docs/index.html` and `docs/legal/` as you ship). Not Optima Sanitas.

## URLs

| Purpose | Value |
|---------|--------|
| **SNS name** | `gameseekrs.sol` |
| **HTTPS (MWA `identity.uri`, reviewers)** | `https://gameseekrs.sol.site` |
| **Code** | `GAMESEEKRS_MWA_IDENTITY_URI` in **`VaultRunner`** / **`GSkrsFlashlight`** `src/config/publicLinks.ts` |
| **Legal HTML + landing** | Public **`gameseekrs-max/Gameseekrs`**: **`docs/index.html`** (line hub) + **`docs/legal/*.html`** — `GAMESEEKRS_LEGAL_PAGES_BASE` in the RN apps points here. |
| **`docs/CNAME`** | Contains **`gameseekrs.sol.site`** for GitHub Pages custom domain (push with `seeker-rampage` → **Gameseekrs**). |

## Wiring checklist

1. Register **`gameseekrs.sol`** on [sns.id](https://sns.id).
2. **Sol.site → Configure:** CNAME to **`gameseekrs-max.github.io`** ([SNS docs](https://docs.sns.id/collection/sns-v2/sol.site/website-configuration.md)).
3. Repo **`gameseekrs-max/Gameseekrs`:** **Settings → Pages → Custom domain** → **`gameseekrs.sol.site`**, enable HTTPS (repo includes **`docs/CNAME`**; GitHub may adjust it when you save).
4. Open **`https://gameseekrs.sol.site/`** — should load **`docs/index.html`** plus **`/legal/…`** paths (same as `gameseekrs-max.github.io/Gameseekrs/` when Pages uses **`/docs`**).

Until DNS resolves, wallet sheets still target **`https://gameseekrs.sol.site`** — finish SNS + Pages before store submission.
