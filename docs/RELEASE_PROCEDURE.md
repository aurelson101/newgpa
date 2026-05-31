# Release Procedure

1. Build and test:

```bash
./scripts/build.sh
```

2. Build AppImage:

```bash
./scripts/build-appimage.sh
```

3. Smoke test:

```bash
./scripts/test-appimage.sh ./dist/NewGPA-x86_64.AppImage
```

4. Sign:

```bash
./scripts/sign-release.sh ./dist/NewGPA-x86_64.AppImage
```

5. Publish:

- AppImage.
- SHA256 file.
- GPG detached signatures.
- Release notes with GnuPG, GPGME, GTK, and libadwaita versions.

