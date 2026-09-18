#!/bin/sh
# Build the native GTK app and install it as a regular Linux program.
set -eu
cd "$(dirname "$0")"

echo "Building Spardame (native)…"
cargo build --release

BIN_DIR="${HOME}/.local/bin"
APP_DIR="${HOME}/.local/share/applications"
ICON_DIR="${HOME}/.local/share/icons/hicolor/256x256/apps"
mkdir -p "$BIN_DIR" "$APP_DIR" "$ICON_DIR"

install -m 755 target/release/spardame "${BIN_DIR}/spardame"

if [ -f ../build/icons/256x256.png ]; then
  install -m 644 ../build/icons/256x256.png "${ICON_DIR}/no.spardame.app.png"
fi

cat > "${APP_DIR}/no.spardame.app.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Spardame
GenericName=Hearts
Comment=Spardame and Hearts
Exec=${BIN_DIR}/spardame
Icon=no.spardame.app
Terminal=false
Categories=Game;CardGame;
Keywords=hearts;cards;spardame;hjerter;
StartupWMClass=no.spardame.app
StartupNotify=true
EOF
chmod 644 "${APP_DIR}/no.spardame.app.desktop"

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "${APP_DIR}" >/dev/null 2>&1 || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
  gtk-update-icon-cache -f "${HOME}/.local/share/icons/hicolor" >/dev/null 2>&1 || true
fi

echo
echo "Installed Spardame (native GTK)."
echo "  program: ${BIN_DIR}/spardame"
echo "  menu:    ${APP_DIR}/no.spardame.app.desktop"
echo
echo "Search for “Spardame” in the app grid, or run:  spardame"
echo "Uninstall: rm -f ${BIN_DIR}/spardame ${APP_DIR}/no.spardame.app.desktop ${ICON_DIR}/no.spardame.app.png"
