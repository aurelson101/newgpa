# NewGPA

NewGPA is a modern Rust/GTK4 privacy assistant inspired by GNU Privacy Assistant and KDE Kleopatra. It provides a portable graphical workflow for OpenPGP keys, X.509/S/MIME certificates, file encryption, signatures, key servers, smartcards, secure local archives, and an explicitly experimental Post-Quantum Lab.

The application delegates real cryptography to standard Linux security tools: GnuPG, gpgsm, gpg-agent, pinentry, and GPGME. Passphrases are never collected by NewGPA widgets.

## Features

- OpenPGP key inventory with graphical selection, refresh, import, public/secret export, ownertrust import/export, key generation, revocation certificate generation, trust changes, disable, and delete actions.
- X.509/S/MIME certificate inventory with import, export, deletion, and chain validation through `gpgsm`.
- File workflows for encryption, decryption, signing, detached signatures, clearsign, and verification.
- Clipboard workflows for encrypting, decrypting, signing, and verifying text.
- Smartcard/token actions for card status, learning card data, card menu, and agent reload.
- Key server actions for WKD lookup, HKP/HKPS search, receive, send, and refresh.
- Secure local vault helper using Unix `0700` folders and symmetric GnuPG archives.
- Post-Quantum Lab status page. This is intentionally experimental and not advertised as OpenPGP-compatible quantum encryption.
- French UI by default, with an English UI available through `NEWGPA_LANG=en newgpa`.

## Fonctionnalités

- Gestion graphique des clés OpenPGP : liste, sélection, actualisation, import, export public/secret, ownertrust, création, révocation, confiance, désactivation et suppression.
- Gestion des certificats X.509/S/MIME via `gpgsm`.
- Chiffrement, déchiffrement, signature et vérification de fichiers.
- Outils presse-papiers pour texte chiffré/signé.
- Support carte à puce/token via GnuPG.
- Recherche WKD et serveurs de clés HKP/HKPS.
- Coffre local simple avec permissions Unix `0700` et archive symétrique GnuPG.
- Laboratoire post-quantique séparé, expérimental et désactivé par défaut.
- Interface française par défaut, interface anglaise avec `NEWGPA_LANG=en newgpa`.

## Build On Ubuntu 26.04

```bash
sudo apt update
sudo apt install build-essential curl pkg-config libgtk-4-dev libadwaita-1-dev libgpgme-dev gnupg2 gpgsm pinentry-curses
curl https://sh.rustup.rs -sSf | sh
```

Build and test:

```bash
./scripts/build.sh
```

Run diagnostics:

```bash
cargo run -- doctor
```

Launch the GUI:

```bash
cargo run
```

Launch the English UI:

```bash
NEWGPA_LANG=en cargo run
```

## AppImage

Install local AppImage tools if needed:

```bash
./scripts/install-appimage-tools.sh
```

Build:

```bash
./scripts/build-appimage.sh
```

Expected output:

```text
dist/NewGPA-x86_64.AppImage
dist/NewGPA-x86_64.AppImage.sha256
```

Smoke test:

```bash
./scripts/test-appimage.sh ./dist/NewGPA-x86_64.AppImage
```

## Security Model

NewGPA does not implement OpenPGP or S/MIME packet cryptography itself. It uses GnuPG/GPGME for compatible operations and relies on `gpg-agent` plus `pinentry` for passphrase prompts.

The Post-Quantum Lab is a research surface only. Stable OpenPGP does not yet standardize ML-KEM/ML-DSA for classic GnuPG keys, so NewGPA keeps these features separate from normal key management.

## Repository Layout

```text
src/
  ui/                 GTK4/libadwaita application
  crypto/             GPGME/OpenPGP/S/MIME boundaries
  keyring/            OpenPGP inventory helpers
  vault/              Secure local vault helpers
  config/             Secure defaults and config paths
  logging/            Structured logging defaults
  security/           Diagnostics and path validation
packaging/
  desktop/            Desktop launcher
  icons/              Application icon
  appstream/          AppStream metadata
scripts/
  build.sh            Release build and tests
  build-appimage.sh   AppDir and AppImage generation
  test-appimage.sh    AppImage smoke test
  sign-release.sh     Hash and detached signatures
```

## Current Limits

- Some GnuPG operations still depend on the user's installed `gpg`, `gpgsm`, `gpg-agent`, and `pinentry`.
- Revocation and smartcard flows may open GnuPG/pinentry prompts depending on the local setup.
- The vault is a pragmatic local archive helper, not a full password manager.
- The Post-Quantum Lab is not production OpenPGP compatibility.
