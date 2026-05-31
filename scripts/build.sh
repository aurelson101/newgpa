#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

export RUSTFLAGS="${RUSTFLAGS:-}"
cargo build --release
cargo test
