# Spardame

Kortspill som **vanlig Linux-program** — eget vindu, ikon i app-menyen, ikke en nettleser-fane.

Spardame (norske regler) og Hearts (Windows). Fire spillere, tre motstandere.

## Installer på Pop!_OS

Last ned fra [Releases](https://github.com/kmrodni-boop/spardame/releases/latest).

**Anbefalt (.deb):**

```bash
sudo dpkg -i spardame_1.0.3_amd64.deb
```

Søk opp **Spardame** i app-menyen og start den som et hvilket som helst annet program.

**Uten sudo (AppImage):**

```bash
chmod +x Spardame-1.0.3.AppImage
./Spardame-1.0.3.AppImage
```

Avinstaller .deb med `sudo apt remove spardame`.

## Bygg fra kildekode

Trenger **Node.js 22**.

```bash
git clone https://github.com/kmrodni-boop/spardame.git
cd spardame
npm install
npm run dist
npm run install:linux
```

`npm run dist` lager filene i `release/`. `npm run install:linux` legger AppImage i `~/.local/bin/spardame` og en snarvei i app-menyen.

Utvikling i eget vindu: `npm run desktop:dev`

## Spill

- **Spardame:** spar dame 100, hjerter ess 20, øvrige hjerter 10, ruter knekt −100. Til 500.
- **Hearts:** hearts 1, queen of spades 13. Shoot the moon gir 26 til de andre. Til 100.
- Engelsk er standard. Bytt til norsk i menyen. Språk og parti lagres på maskinen.

## English

A native Linux desktop app (window + icon), not a browser tab.

Download the `.deb` or AppImage from [Releases](https://github.com/kmrodni-boop/spardame/releases/latest), then:

```bash
sudo dpkg -i spardame_1.0.3_amd64.deb
```
