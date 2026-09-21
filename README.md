# Spardame

Kortspill som **vanlig Linux-program** — eget vindu, ikon i app-menyen. Native GTK 4 (Rust), ingen Electron/Chromium.

To varianter: **Spardame** (norske regler, til 500) og **Hearts** (Windows, til 100). Fire spillere, tre motstandere. Engelsk er standard; bytt til norsk i menyen.

| Pakke | Typisk størrelse | Når |
|---|---|---|
| **`.deb`** | **~0,3 MB** (708 KB installert) | Pop!_OS / Ubuntu — bruker GTK som allerede er installert |
| **Flatpak** | app ~5–15 MB + GNOME-runtime | Portable, sandkasse, «installer som et program» |
| **AppImage** | ~36 MB | Én fil; bundler GTK 4 selv, derfor størst |

## Last ned

Fra [Releases](https://github.com/kmrodni-boop/spardame/releases/latest):

**Anbefalt på Pop!_OS — .deb:**

```bash
sudo dpkg -i spardame_1.2.0_amd64.deb
```

**Flatpak:**

```bash
flatpak install --user Spardame-1.2.0.flatpak
flatpak run no.spardame.app
```

**AppImage:**

```bash
chmod +x Spardame-1.2.0-x86_64.AppImage
./Spardame-1.2.0-x86_64.AppImage
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

## Spill

- **Spardame:** spar dame 100, hjerter ess 20, øvrige hjerter 10, ruter knekt −100. Til 500.
- **Hearts:** hearts 1, queen of spades 13. Shoot the moon gir 26 til de andre. Til 100.

## English

Native GTK 4 Hearts for Linux. Norwegian Spardame rules (to 500) and Windows-style Hearts (to 100). Prefer the `.deb` on Pop!_OS (~a few MB, uses system GTK) or the Flatpak.
