#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARCH="${ARCH:-$(uname -m)}"
case "$ARCH" in
  x86_64|amd64) APPIMAGE_ARCH="x86_64" ;;
  aarch64|arm64) APPIMAGE_ARCH="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

TOOLS_DIR="$ROOT/tools"
mkdir -p "$TOOLS_DIR"

LINUXDEPLOY_URL="${LINUXDEPLOY_URL:-https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-${APPIMAGE_ARCH}.AppImage}"
APPIMAGETOOL_URL="${APPIMAGETOOL_URL:-https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-${APPIMAGE_ARCH}.AppImage}"

download() {
  local url="$1"
  local output="$2"
  if command -v curl >/dev/null 2>&1; then
    curl --fail --location --show-error "$url" --output "$output"
  elif command -v wget >/dev/null 2>&1; then
    wget -O "$output" "$url"
  else
    echo "curl or wget is required" >&2
    exit 1
  fi
  chmod +x "$output"
}

download "$LINUXDEPLOY_URL" "$TOOLS_DIR/linuxdeploy"
download "$APPIMAGETOOL_URL" "$TOOLS_DIR/appimagetool"

echo "Installed AppImage tools in $TOOLS_DIR"
echo "Run: PATH=\"$TOOLS_DIR:\$PATH\" ./scripts/build-appimage.sh"

