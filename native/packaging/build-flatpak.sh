#!/bin/sh
# Build a Flatpak bundle. Requires flatpak-builder and the GNOME 48 runtime.
set -eu
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname "$0")" && pwd)"
NATIVE="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
ROOT="$(CDPATH= cd -- "$NATIVE/.." && pwd)"
VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$NATIVE/Cargo.toml" | head -n1)"
DIST="$NATIVE/dist"
BUILD="$NATIVE/flatpak/build-dir"
MANIFEST="$NATIVE/flatpak/no.spardame.app.yml"
BUNDLE="$DIST/Spardame-${VERSION}.flatpak"

if ! command -v flatpak-builder >/dev/null 2>&1; then
  echo "Install: sudo apt install flatpak flatpak-builder" >&2
  echo "Then:    flatpak remote-add --if-not-exists --user flathub https://dl.flathub.org/repo/flathub.flatpakrepo" >&2
  echo "         flatpak install --user flathub org.gnome.Platform//48 org.gnome.Sdk//48 org.freedesktop.Sdk.Extension.rust-stable" >&2
  exit 1
fi

mkdir -p "$DIST"
cd "$ROOT"
flatpak-builder --force-clean --user --install-deps-from=flathub \
  --repo="$DIST/flatpak-repo" "$BUILD" "$MANIFEST"
flatpak build-bundle "$DIST/flatpak-repo" "$BUNDLE" no.spardame.app
echo "Wrote $BUNDLE"
echo "Install with:  flatpak install --user $BUNDLE"
echo "Run with:      flatpak run no.spardame.app"
