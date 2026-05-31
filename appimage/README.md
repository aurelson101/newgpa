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

## French UI / English UI

French is the default interface. To launch the English interface:

```bash
NEWGPA_LANG=en ./NewGPA-x86_64.AppImage
```
