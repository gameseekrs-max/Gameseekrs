# Privacy Policy — GameSeekrs: GSkrs Flashlight

**Last updated:** April 26, 2026

## Who we are

**GameSeekrs** publishes **GSkrs Flashlight**, a pocket **utility** (torch, strobe, Clip & Pick) **for [Solana Mobile Seeker](https://solanamobile.com/seeker)** first—Solana Mobile’s **Android** handset. Some LED / camera paths may work on other **Android** phones, but tuning, testing, and store intent focus on **Seeker**. We ship **Android** only (no iOS app for this title). We do **not** run a separate product website; quick updates and contact are on **X**: [@gameseekrs](https://x.com/gameseekrs).

This policy covers **only** the **GSkrs Flashlight** mobile application (React Native; client source may be private). It **complements** the **Vault Runner** policy and on-chain program disclosure in the same public repository: **[PRIVACY.md](../PRIVACY.md)**.

**Companion product — same suite, different app:** **Vault Runner** is our arcade game from the same publisher; see **[PRIVACY.md](../PRIVACY.md)** for wallet and on-chain practices specific to gameplay. **Do not** read the companion app policy as a promise of any live reward pool or “daily” distribution until the Vault program and terms on **mainnet** are published.

## What the app does (summary)

- Rear **flashlight** and **strobe** (including a “turbo” path that may release the camera to drive the LED faster).
- **Camera** preview, optional **short video** recording, and **Clip & Pick** (record a brief clip, choose one frame to keep as an image; discard the rest locally).
- Optional **Solana Mobile Wallet Adapter** connect / sign flows for testing or future features.

## What we collect

- **Camera and microphone:** The app uses the **camera** for preview, torch control, recording, and frame extraction. **Audio recording is disabled** for the built-in clip path (`enableAudio: false`); if a future build enables microphone use, this file will be updated and the store listing will reflect it.
- **Files on device:** Clips and extracted frames are written to the app’s **cache / temporary storage** on your phone. You choose whether to keep one exported image or delete material from the device. We do **not** upload camera or clip data to GameSeekrs servers **unless** a future version explicitly adds cloud or blockchain upload — if that happens, this policy will be updated before release.
- **Wallet (optional):** If you connect a wallet via Mobile Wallet Adapter, your **wallet public key** may be used for signing or display. We do **not** receive your seed phrase or private key; signing happens in your wallet (e.g. Seed Vault). Any **on-chain** transactions you approve are **public on Solana** like any blockchain activity.
- **Vibration:** Short vibration may be used for light feedback where implemented.
- **Device model (optional):** The app may read **device model / brand** (e.g. to tune **Seeker**-class behavior). This is not used to identify you personally.

## What we do not do

- No sale of personal data.
- No GameSeekrs **account** system beyond your self-custodial wallet (when you use wallet features).
- No in-app ad analytics SDK in the reference build described by this policy; if that changes, this document will be updated on GitHub.

## Children

GSkrs Flashlight is not directed at children under 13 (or the age required in your jurisdiction). Wallet and camera features are intended for users who can lawfully consent to device permissions and wallet terms.

## Third parties

- **Solana RPC:** If you use wallet features, your app may send transactions and read chain state via Solana RPC endpoints configured in the app. Those providers see IP and request metadata under **their** policies.
- **X (Twitter):** If you open our X profile from a link inside the app, X’s privacy policy applies there.
- **Google / Android:** The operating system and Play Services (if present) may collect diagnostics per Google’s policies; see [Google Privacy Policy](https://policies.google.com/privacy).

## Camera privacy default

The app is designed so the **camera session does not start** until you take an action that needs it (e.g. enabling live preview, steady light, strobe, recording, or Clip & Pick). Granting OS “camera permission” alone does **not** imply continuous background streaming by this policy’s intent; behavior is as implemented in the build you install.

## Changes

We may update this policy by committing a new version to this **public** GitHub repository. The “Last updated” date at the top of this file will change.

## Contact

- **X:** [@gameseekrs](https://x.com/gameseekrs)
- **This policy (GSkrs Flashlight):** [docs/PRIVACY_GSKRS_FLASHLIGHT.md](https://github.com/gameseekrs-max/Gameseekrs/blob/main/docs/PRIVACY_GSKRS_FLASHLIGHT.md) (update path if your default branch or folder layout differs).
- **Vault Runner / program:** [PRIVACY.md](https://github.com/gameseekrs-max/Gameseekrs/blob/main/PRIVACY.md)

---

*Optima Sanitas / `sanitas_seeker` materials in separate reference archives are **not** this product. Do not mix compliance or keys between those projects and **GameSeekrs** apps.*
