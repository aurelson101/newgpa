# User Guide

NewGPA is a modern interface for OpenPGP keys, S/MIME certificates, encrypted files, signatures, and future secure local vaults.

## Principles

- Passwords and passphrases are handled by `pinentry`.
- Network access is disabled by default.
- The Post-Quantum Lab is experimental and disabled by default.

## Start

Launch the application:

```bash
newgpa
```

Run diagnostics:

```bash
newgpa doctor
```

## Sections

- My Keys: OpenPGP public and private keys.
- Certificates: X.509/S/MIME certificates.
- Encrypt: file encryption and recipient selection.
- Decrypt: decryption through GnuPG.
- Sign: attached, detached, and clearsign signatures.
- Verify: signature reports.
- Secure Vault: planned local encrypted space.
- Post-Quantum Lab: experimental, not standard OpenPGP compatible.
- Settings: security, networking, proxy, appearance.

