# Threat Model

## Assets

- Private keys managed by GnuPG.
- Passphrases handled by gpg-agent and pinentry.
- Plaintext files selected by the user.
- Encrypted outputs, signatures, reports, and local vault data.
- Certificate trust decisions.

## In Scope

- Accidental plaintext leaks through logs.
- Unsafe temporary files.
- Path traversal in file handling.
- Confusing verification results.
- Unintended network access.
- Unsafe export of private keys.
- AppImage release tampering.

## Out of Scope

- Compromised kernel or desktop session.
- Malicious GnuPG binary.
- Hardware key firmware compromise.
- Cryptanalysis of standard algorithms.
- Post-quantum production guarantees.

## Controls

- Delegate passphrases to pinentry.
- Use GPGME/GnuPG for OpenPGP and S/MIME.
- Restrictive config permissions.
- Redacted logging.
- Explicit network opt-in.
- Release checksums and signatures.
- Experimental feature isolation.

