# Spardame

Kortspill som **vanlig Linux-program** — eget vindu, ikon i app-menyen.

To varianter: **Spardame** (norske regler, til 500) og **Hearts** (Windows, til 100). Fire spillere, tre motstandere. Engelsk er standard; bytt til norsk i menyen.

Det finnes to utgaver i dette repoet:

| Utgave | Mappe | Typisk størrelse | Anbefalt |
|---|---|---|---|
| **Native (GTK4 / Rust)** | `native/` | **~2–4 MB** | Ja — ekte Linux-program |
| Electron (Chromium) | rot + `electron/` | ~126–168 MB | Bare om GTK-bygget ikke går |

## Native (anbefalt)

Trenger GTK 4 og libadwaita, som allerede ligger på Pop!_OS.

```bash
sudo apt install build-essential cargo libgtk-4-dev libadwaita-1-dev
git clone https://github.com/kmrodni-boop/spardame.git
cd spardame/native
./install.sh
```

Søk opp **Spardame** i app-menyen. Avinstaller med linjen `install.sh` skriver ut til slutt.

Utvikling:

```bash
cd native
cargo run
cargo test --no-default-features
```

Lagret parti og innstillinger ligger i `~/.local/share/spardame/save.json`.

## Electron (.deb / AppImage)

Hvis du vil ha den ferdigpakkede Chromium-utgaven: last ned fra [Releases](https://github.com/kmrodni-boop/spardame/releases/latest).

```bash
sudo dpkg -i spardame_1.0.4_amd64.deb
```

Uten sudo: `chmod +x Spardame-1.0.4.AppImage && ./Spardame-1.0.4.AppImage`

Bygg selv (Node.js 22): `npm install && npm run dist && npm run install:linux`

## Spill

- **Spardame:** spar dame 100, hjerter ess 20, øvrige hjerter 10, ruter knekt −100. Til 500.
- **Hearts:** hearts 1, queen of spades 13. Shoot the moon gir 26 til de andre. Til 100.

## English

A native Linux desktop app. Prefer the GTK build in `native/` (~a few MB). The Electron AppImage is a fallback and ships Chromium (~168 MB).

```bash
sudo apt install build-essential cargo libgtk-4-dev libadwaita-1-dev
cd native && ./install.sh
```
