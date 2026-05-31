#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_NAME="NewGPA"
ARCH="${ARCH:-$(uname -m)}"
case "$ARCH" in
  x86_64|amd64) APPIMAGE_ARCH="x86_64" ;;
  aarch64|arm64) APPIMAGE_ARCH="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

cd "$ROOT"
export PATH="$ROOT/tools:$PATH"
cargo build --release --locked

APPDIR="$ROOT/dist/${APP_NAME}.AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/share/applications" "$APPDIR/usr/share/icons/hicolor/scalable/apps" "$APPDIR/usr/share/metainfo"

install -Dm755 "$ROOT/target/release/newgpa" "$APPDIR/usr/bin/newgpa"
install -Dm644 "$ROOT/packaging/desktop/org.newgpa.NewGPA.desktop" "$APPDIR/usr/share/applications/org.newgpa.NewGPA.desktop"
install -Dm644 "$ROOT/packaging/icons/org.newgpa.NewGPA.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/org.newgpa.NewGPA.svg"
install -Dm644 "$ROOT/packaging/appstream/org.newgpa.NewGPA.metainfo.xml" "$APPDIR/usr/share/metainfo/org.newgpa.NewGPA.metainfo.xml"

cat > "$APPDIR/AppRun" <<'APPRUN'
#!/usr/bin/env bash
set -euo pipefail
HERE="$(dirname "$(readlink -f "$0")")"
export PATH="$HERE/usr/bin:$PATH"
exec "$HERE/usr/bin/newgpa" "$@"
APPRUN
chmod +x "$APPDIR/AppRun"
ln -sf usr/share/applications/org.newgpa.NewGPA.desktop "$APPDIR/org.newgpa.NewGPA.desktop"
ln -sf usr/share/icons/hicolor/scalable/apps/org.newgpa.NewGPA.svg "$APPDIR/org.newgpa.NewGPA.svg"

if ! command -v linuxdeploy >/dev/null 2>&1; then
  echo "linuxdeploy is required. Download it from https://github.com/linuxdeploy/linuxdeploy/releases" >&2
  exit 1
fi
if ! command -v appimagetool >/dev/null 2>&1; then
  echo "appimagetool is required. Download it from https://github.com/AppImage/AppImageKit/releases" >&2
  exit 1
fi

linuxdeploy --appdir "$APPDIR" --desktop-file "$APPDIR/usr/share/applications/org.newgpa.NewGPA.desktop" --icon-file "$APPDIR/usr/share/icons/hicolor/scalable/apps/org.newgpa.NewGPA.svg"
OUTPUT="$ROOT/dist/${APP_NAME}-${APPIMAGE_ARCH}.AppImage"
rm -f "$OUTPUT" "$OUTPUT.sha256"
ARCH="$APPIMAGE_ARCH" appimagetool "$APPDIR" "$OUTPUT"
sha256sum "$OUTPUT" > "$OUTPUT.sha256"
echo "Built $OUTPUT"
