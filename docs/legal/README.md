# GameSeekrs — public legal HTML (legacy `docs/legal`)

> **Preferred hub:** dedicated repo **`gameseekrs-max/gameseekrs-site`** — root **`index.html`**, **`vault-runner/`**, **`flashlight/`**, **`assets/legal.css`**. See **`../MIGRATION_GAMESEEKRS_SITE.md`**.

These static pages (`*.html` + `base.css`) remain for **backward compatibility** until apps and store URLs point at **gameseekrs-site** — program repo plus separate static legal hub.

**Legacy: GitHub Pages** on **`gameseekrs-max/Gameseekrs`** (folder **`/docs`**):

1. **Settings** → **Pages** → **Build and deployment**
2. **Source:** Deploy from a branch
3. **Branch:** `main` (or your default) → folder **`/docs`**
4. **Save**

Live base URLs (same deployment after custom domain is set):

- **SNS + Sol.site (reviewers / MWA identity):** **`https://gameseekrs.sol.site`**
- **GitHub Pages default:** **`https://gameseekrs-max.github.io/Gameseekrs/`**

**Landing page:** **`../index.html`** at the site root — both apps, disclaimer, legal links, and official URLs.

Legal routes (legacy paths; prefer **`gameseekrs.sol.site`** hub URLs for new listings):

| File | Use |
|------|-----|
| `legal/vault-privacy.html` | Vault Runner — Privacy |
| `legal/vault-terms.html` | Vault Runner — Terms |
| `legal/flashlight-privacy.html` | GSkrs Flashlight — Privacy |
| `legal/flashlight-terms.html` | GSkrs Flashlight — Terms |
| `legal/license.html` | Shared MIT license |

**Sync:** When you change the markdown in this repo’s root or `docs/*.md`, update the matching HTML (or copy from a future generator). **Authoritative** legal text in Git remains the **`.md`** files; these HTML files are the **formatted** public view.

Apps should open **HTTPS** pages users can load without signing in to GitHub — prefer the **`gameseekrs-site`** hub at **`gameseekrs.sol.site`** once migration is complete.
