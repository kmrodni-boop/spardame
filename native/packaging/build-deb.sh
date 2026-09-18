#!/bin/sh
# Build a tiny .deb that uses the system's GTK 4 + libadwaita.
set -eu
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname "$0")" && pwd)"
NATIVE="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
ROOT="$(CDPATH= cd -- "$NATIVE/.." && pwd)"
cd "$NATIVE"

VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n1)"
ARCH="$(dpkg --print-architecture 2>/dev/null || echo amd64)"
PKG="spardame_${VERSION}_${ARCH}"
DIST="$NATIVE/dist"
STAGE="$DIST/$PKG"

echo "Building native binary…"
cargo build --release --locked

rm -rf "$STAGE"
mkdir -p "$STAGE/DEBIAN"
"$SCRIPT_DIR/install-tree.sh" "$STAGE"

SIZE="$(du -sk "$STAGE/usr" | awk '{print $1}')"
cat > "$STAGE/DEBIAN/control" <<EOF
Package: spardame
Version: $VERSION
Section: games
Priority: optional
Architecture: $ARCH
Depends: libgtk-4-1, libadwaita-1-0, libc6
Maintainer: Kai Marius Alm <235362284+kmrodni-boop@users.noreply.github.com>
Homepage: https://github.com/kmrodni-boop/spardame
Installed-Size: $SIZE
Description: Spardame and Hearts card game
 Native GTK 4 Hearts for Linux. Norwegian Spardame rules (to 500) and
 Windows Hearts (to 100). Four players, three computer opponents.
EOF

dpkg-deb --root-owner-group --build "$STAGE" "$DIST/${PKG}.deb"
rm -rf "$STAGE"
echo "Wrote $DIST/${PKG}.deb"
