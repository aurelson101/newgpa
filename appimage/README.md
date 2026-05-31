# NewGPA AppImage

This folder contains the public portable Linux build of NewGPA.

## Download

- `NewGPA-x86_64.AppImage`
- `NewGPA-x86_64.AppImage.sha256`

## Run

```bash
chmod +x NewGPA-x86_64.AppImage
./NewGPA-x86_64.AppImage
```

## Verify

```bash
sha256sum -c NewGPA-x86_64.AppImage.sha256
```

## Language

NewGPA detects the system language at startup. French locales use French, and all other locales use English by default.

Use the language button in the top-left corner to switch between French and English.
