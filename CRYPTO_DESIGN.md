# Crypto Design

## Standard Crypto

OpenPGP and S/MIME operations are delegated to GPGME. GPGME selects the GnuPG engine:

- OpenPGP via `gpg`.
- S/MIME/CMS via `gpgsm`.
- Passphrases via `gpg-agent` and `pinentry`.

NewGPA must not parse or implement packet-level OpenPGP encryption itself.

## Post-Quantum Lab

The Post-Quantum Lab is not standard OpenPGP. It is disabled by default and must show a clear warning.

Candidate algorithms:

- ML-KEM for key encapsulation.
- ML-DSA for signatures.
- SLH-DSA for hash-based signatures.

Future implementation should use reviewed libraries such as liboqs. Hybrid modes must keep standard OpenPGP output separate from experimental metadata unless an official standard is available.

## Temporary Files

Temporary archives and decrypted intermediates must be created with restrictive permissions, cleaned after use, and excluded from logs.

