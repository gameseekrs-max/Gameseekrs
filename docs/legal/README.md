# GameSeekrs — public legal HTML (GitHub Pages)

These static pages (`*.html` + `base.css`) are the **in-app and store** targets for **Privacy**, **Terms**, and **License**, following the same pattern as **Seeker Mobile Calc** (`optimasanitas.github.io/seeker-mobile-calc-legal/…`).

**Enable GitHub Pages** on **`gameseekrs-max/Gameseekrs`**:

1. **Settings** → **Pages** → **Build and deployment**
2. **Source:** Deploy from a branch
3. **Branch:** `main` (or your default) → folder **`/docs`**
4. **Save**

Live base URL:

**`https://gameseekrs-max.github.io/Gameseekrs/`**

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
