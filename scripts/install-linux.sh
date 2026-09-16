#!/bin/sh
# Install the built AppImage into the current user's application menu.
set -eu
cd "$(dirname "$0")/.."

APPIMAGE=$(ls -1 release/Spardame*.AppImage 2>/dev/null | head -n 1 || true)
if [ -z "$APPIMAGE" ]; then
  echo "No AppImage found. Run: npm run dist" >&2
  exit 1
fi

BIN_DIR="${HOME}/.local/bin"
APP_DIR="${HOME}/.local/share/applications"
ICON_DIR="${HOME}/.local/share/icons/hicolor/256x256/apps"
mkdir -p "$BIN_DIR" "$APP_DIR" "$ICON_DIR"

DEST="${BIN_DIR}/spardame"
cp "$APPIMAGE" "$DEST"
chmod +x "$DEST"

if [ -f build/icons/256x256.png ]; then
  cp build/icons/256x256.png "${ICON_DIR}/spardame.png"
fi

cat > "${APP_DIR}/spardame.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Spardame
Comment=Spardame and Hearts
Exec=${DEST}
Icon=spardame
Terminal=false
Categories=Game;CardGame;
StartupWMClass=spardame
EOF

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "${APP_DIR}" >/dev/null 2>&1 || true
fi

echo "Installed Spardame."
echo "  launcher: ${DEST}"
echo "  menu:     ${APP_DIR}/spardame.desktop"
echo "Search for “Spardame” in the app grid, or run: spardame"
