# Security Policy

## Supported Security Model

NewGPA delegates OpenPGP and S/MIME to GnuPG through GPGME. Passphrases must be handled by `gpg-agent` and `pinentry`, not by NewGPA widgets.

## High Security Defaults

- Network access is disabled by default.
- Post-Quantum Lab is disabled by default.
- Config files use restrictive permissions where supported.
- Logs must not contain passphrases, private key material, plaintext file contents, or secret filenames when high-security masking is enabled.

## Reporting Issues

Report vulnerabilities privately to the maintainer. Include reproduction steps, platform, GnuPG/GPGME versions, and whether an AppImage or development build was used.

## Release Integrity

Release artifacts should include:

- AppImage file.
- SHA256 checksum.
- Detached GPG signature for the AppImage.
- Detached GPG signature for the checksum file.

