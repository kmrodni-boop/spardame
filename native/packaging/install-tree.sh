#!/bin/sh
# Copy the native app into a prefix (usr-style layout).
# Usage: install-tree.sh DEST
#   DEST/usr/bin/spardame
#   DEST/usr/share/applications/...
set -eu
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname "$0")" && pwd)"
NATIVE="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
ROOT="$(CDPATH= cd -- "$NATIVE/.." && pwd)"
DEST="${1:?destination prefix}"

BIN="$NATIVE/target/release/spardame"
if [ ! -x "$BIN" ]; then
  echo "Missing $BIN — run: cargo build --release" >&2
  exit 1
fi

install -Dm755 "$BIN" "$DEST/usr/bin/spardame"
install -Dm644 "$NATIVE/data/no.spardame.app.desktop" \
  "$DEST/usr/share/applications/no.spardame.app.desktop"
install -Dm644 "$NATIVE/data/no.spardame.app.metainfo.xml" \
  "$DEST/usr/share/metainfo/no.spardame.app.metainfo.xml"

for size in 64 128 256 512; do
  src="$ROOT/build/icons/${size}x${size}.png"
  if [ -f "$src" ]; then
    install -Dm644 "$src" \
      "$DEST/usr/share/icons/hicolor/${size}x${size}/apps/no.spardame.app.png"
  fi
done
