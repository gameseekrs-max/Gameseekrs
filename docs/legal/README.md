# GameSeekrs — public legal HTML (legacy `docs/legal`)

> **Preferred hub:** dedicated repo **`gameseekrs-max/gameseekrs-site`** — root **`index.html`**, **`vault-runner/`**, **`flashlight/`**, **`assets/legal.css`**. See **`../MIGRATION_GAMESEEKRS_SITE.md`**.

These static pages (`*.html` + `base.css`) remain for **backward compatibility** until apps and store URLs point at **gameseekrs-site**. Same pattern as Optima’s split between program repo and **`optimasanitas-site`**.

**Legacy: GitHub Pages** on **`gameseekrs-max/Gameseekrs`** (folder **`/docs`**):

1. **Settings** → **Pages** → **Build and deployment**
2. **Source:** Deploy from a branch
3. **Branch:** `main` (or your default) → folder **`/docs`**
4. **Save**

Live base URLs (same deployment after custom domain is set):

- **SNS + Sol.site (reviewers / MWA identity):** **`https://gameseekrs.sol.site`**
- **GitHub Pages default:** **`https://gameseekrs-max.github.io/Gameseekrs/`**

**Landing page:** **`../index.html`** at the site root — both apps, disclaimer, legal links, and official URLs (same pattern as Optima’s public legal repo).

Legal routes (used by **VaultRunner** and **GSkrsFlashlight** `publicLinks.ts`):

| File | Use |
|------|-----|
| `legal/vault-privacy.html` | Vault Runner — Privacy |
| `legal/vault-terms.html` | Vault Runner — Terms |
| `legal/flashlight-privacy.html` | GSkrs Flashlight — Privacy |
| `legal/flashlight-terms.html` | GSkrs Flashlight — Terms |
| `legal/license.html` | Shared MIT license |

**Sync:** When you change the markdown in this repo’s root or `docs/*.md`, update the matching HTML (or copy from a future generator). **Authoritative** legal text in Git remains the **`.md`** files; these HTML files are the **formatted** public view.

**Private app repos** (Vault Runner, GSkrs Flashlight) use **`publicLinks.ts`** with `GAMESEEKRS_LEGAL_PAGES_BASE` + paths above.
