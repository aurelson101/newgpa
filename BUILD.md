# Build Guide

## Ubuntu 26.04 and 24.04

Install dependencies:

```bash
sudo apt update
sudo apt install build-essential curl pkg-config libgtk-4-dev libadwaita-1-dev libgpgme-dev gnupg2 gpgsm pinentry-curses
curl https://sh.rustup.rs -sSf | sh
```

Build:

```bash
./scripts/build.sh
```

If the GUI dependencies are not available, build the CLI diagnostics only:

```bash
cargo build --release --no-default-features
```

## Fedora

```bash
sudo dnf install gcc pkgconf-pkg-config gtk4-devel libadwaita-devel gpgme-devel gnupg2 pinentry rust cargo
```

## Arch Linux

```bash
sudo pacman -S base-devel pkgconf gtk4 libadwaita gpgme gnupg pinentry rust
```

## openSUSE

```bash
sudo zypper install gcc pkgconf-pkg-config gtk4-devel libadwaita-devel gpgme-devel gpg2 pinentry rust cargo
```

