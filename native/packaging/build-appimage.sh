#!/bin/sh
# Bundle the native GTK binary as an AppImage (linuxdeploy + gtk plugin).
# GTK 4 AppImages are bulkier than the .deb; prefer .deb or Flatpak on Pop!_OS.
set -eu
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname "$0")" && pwd)"
NATIVE="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
cd "$NATIVE"

VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n1)"
DIST="$NATIVE/dist"
APPDIR="$DIST/AppDir"
CACHE="${XDG_CACHE_HOME:-$HOME/.cache}/spardame-packaging"
mkdir -p "$DIST" "$CACHE"

echo "Building native binary…"
cargo build --release --locked

rm -rf "$APPDIR"
"$SCRIPT_DIR/install-tree.sh" "$APPDIR"

# AppImage tools expect files at AppDir/usr/… which install-tree already uses.
LINUXDEPLOY="$CACHE/linuxdeploy-x86_64.AppImage"
GTK_PLUGIN="$CACHE/linuxdeploy-plugin-gtk.sh"
if [ ! -x "$LINUXDEPLOY" ]; then
  echo "Downloading linuxdeploy…"
  curl -fsSL -o "$LINUXDEPLOY" \
    "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage"
  chmod +x "$LINUXDEPLOY"
fi
if [ ! -x "$GTK_PLUGIN" ]; then
  curl -fsSL -o "$GTK_PLUGIN" \
    "https://raw.githubusercontent.com/linuxdeploy/linuxdeploy-plugin-gtk/master/linuxdeploy-plugin-gtk.sh"
  chmod +x "$GTK_PLUGIN"
fi

# linuxdeploy looks for plugins next to itself or on PATH.
cp "$GTK_PLUGIN" "$CACHE/linuxdeploy-plugin-gtk.sh"
export PATH="$CACHE:$PATH"
export LINUXDEPLOY_OUTPUT_VERSION="$VERSION"
export OUTPUT="$DIST/Spardame-${VERSION}-x86_64.AppImage"
export ARCH=x86_64
export UPDATE_INFORMATION=""
export LDAI_OUTPUT="$OUTPUT"

cd "$CACHE"
./linuxdeploy-x86_64.AppImage \
  --appdir "$APPDIR" \
  --executable "$APPDIR/usr/bin/spardame" \
  --desktop-file "$APPDIR/usr/share/applications/no.spardame.app.desktop" \
  --icon-file "$APPDIR/usr/share/icons/hicolor/256x256/apps/no.spardame.app.png" \
  --plugin gtk \
  --output appimage

# linuxdeploy may write the AppImage in the current directory.
if [ -f "$CACHE/Spardame-${VERSION}-x86_64.AppImage" ]; then
  mv -f "$CACHE/Spardame-${VERSION}-x86_64.AppImage" "$OUTPUT"
fi
if [ -f "$CACHE/no.spardame.app-${VERSION}-x86_64.AppImage" ]; then
  mv -f "$CACHE/no.spardame.app-${VERSION}-x86_64.AppImage" "$OUTPUT"
fi

echo "Wrote $OUTPUT"
