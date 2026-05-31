#!/usr/bin/env bash
set -euo pipefail

ARTIFACT="${1:-}"
if [[ -z "$ARTIFACT" || ! -f "$ARTIFACT" ]]; then
  echo "Usage: $0 ./dist/NewGPA-x86_64.AppImage" >&2
  exit 1
fi

sha256sum "$ARTIFACT" > "${ARTIFACT}.sha256"
gpg --detach-sign --armor "$ARTIFACT"
gpg --detach-sign --armor "${ARTIFACT}.sha256"

