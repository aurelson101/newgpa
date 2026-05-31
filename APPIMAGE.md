# AppImage Guide

Build:

```bash
./scripts/build-appimage.sh
```

Install local AppImage tooling first if needed:

```bash
./scripts/install-appimage-tools.sh
```

Expected output:

```text
dist/NewGPA-x86_64.AppImage
dist/NewGPA-x86_64.AppImage.sha256
```

For ARM64:

```bash
ARCH=aarch64 ./scripts/build-appimage.sh
```

NewGPA should bundle GUI dependencies that are not guaranteed across distributions, while still integrating with system GnuPG, `gpg-agent`, and `pinentry`. This avoids isolating private key prompts from the user's configured agent.

Smoke test:

```bash
./scripts/test-appimage.sh ./dist/NewGPA-x86_64.AppImage
```

Sign a release:

```bash
./scripts/sign-release.sh ./dist/NewGPA-x86_64.AppImage
```
