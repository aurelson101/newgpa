#!/usr/bin/env bash
set -euo pipefail

APPIMAGE="${1:-}"
if [[ -z "$APPIMAGE" || ! -x "$APPIMAGE" ]]; then
  echo "Usage: $0 ./dist/NewGPA-x86_64.AppImage" >&2
  exit 1
fi

"$APPIMAGE" --appimage-extract-and-run doctor

