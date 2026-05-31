# Architecture

NewGPA uses a layered design:

- UI layer: GTK4/libadwaita, workflows, validation, and user-facing status.
- Service layer: keyring, vault, network policy, and verification reports.
- Crypto boundary: GPGME contexts for OpenPGP and CMS/S/MIME.
- Security layer: environment diagnostics, path validation, permissions, and logging rules.
- Packaging layer: desktop files, AppStream metadata, AppDir, AppImage, checksums, and signatures.

## Crypto Boundary

The application treats GPGME as the trust boundary for OpenPGP and S/MIME operations. UI code should never handle passphrases or private key bytes.

## Network Boundary

Network operations such as WKD, HKPS, URL import, and key refresh must require explicit user intent when high-security mode is enabled.

## Experimental Boundary

Post-Quantum Lab features must be compiled or enabled separately, clearly marked, and kept out of default OpenPGP/S/MIME workflows.

