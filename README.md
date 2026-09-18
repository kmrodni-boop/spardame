# Spardame

Kortspill som **vanlig Linux-program** — eget vindu, ikon i app-menyen.

To varianter: **Spardame** (norske regler, til 500) og **Hearts** (Windows, til 100). Fire spillere, tre motstandere. Engelsk er standard; bytt til norsk i menyen.

Native GTK 4 (Rust) er den vanlige utgaven. Electron-bygget med Chromium ligger igjen som reserve.

| Pakke | Typisk størrelse | Når |
|---|---|---|
| **`.deb` (native)** | **~2–5 MB** | Pop!_OS / Ubuntu — bruker GTK som allerede er installert |
| **Flatpak** | app ~5–15 MB + GNOME-runtime | Portable, sandkasse, «installer som et program» |
| **AppImage (native)** | ~20–50 MB | Én fil; GTK 4-AppImages kan være lunefulle |
| Electron AppImage/.deb | ~126–168 MB | Bare om native GTK ikke går |

## Last ned

Fra [Releases](https://github.com/kmrodni-boop/spardame/releases/latest) (native 1.1):

**Anbefalt på Pop!_OS — .deb:**

```bash
sudo dpkg -i spardame_1.1.0_amd64.deb
```

**Flatpak:**

```bash
flatpak install --user Spardame-1.1.0.flatpak
flatpak run no.spardame.app
```

**AppImage:**

```bash
chmod +x Spardame-1.1.0-x86_64.AppImage
./Spardame-1.1.0-x86_64.AppImage
```

Søk opp **Spardame** i app-menyen etter .deb eller Flatpak.

## Bygg selv

```bash
sudo apt install build-essential cargo libgtk-4-dev libadwaita-1-dev
git clone https://github.com/kmrodni-boop/spardame.git
cd spardame/native
./install.sh                          # rett i ~/.local
# eller pakker:
./packaging/build-deb.sh              # native/dist/*.deb
./packaging/build-flatpak.sh          # krever flatpak-builder + GNOME 48
./packaging/build-appimage.sh         # krever curl / linuxdeploy
```

Utvikling: `cargo run` · tester: `cargo test --no-default-features`

Lagret parti: `~/.local/share/spardame/save.json` (Flatpak: `~/.var/app/no.spardame.app/data/spardame/`).

## Electron (Chromium, ~168 MB)

```bash
sudo dpkg -i spardame_1.0.4_amd64.deb
```

Bygg: `npm install && npm run dist`

## Spill

- **Spardame:** spar dame 100, hjerter ess 20, øvrige hjerter 10, ruter knekt −100. Til 500.
- **Hearts:** hearts 1, queen of spades 13. Shoot the moon gir 26 til de andre. Til 100.

## English

Native GTK 4 Hearts. Prefer the `.deb` on Pop!_OS (~a few MB, uses system GTK) or the Flatpak. The Electron AppImage ships Chromium (~168 MB).
