# Spardame

Kortspill for nettleseren: **Spardame** (norske regler) og **Hjerter** (Windows Hearts). Fire spillere, tre motstandere, offisielle regler.

English is the default language. Switch to Norwegian in the menu.

## På Pop!_OS / VS Code

Du trenger **Node.js 22** (npm følger med).

```bash
# hvis du ikke har Node 22 ennå (nvm):
# curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
# nvm install 22

git clone https://github.com/kmrodni-boop/spardame.git
cd spardame
npm install
npm run dev
```

Åpne [http://localhost:8080](http://localhost:8080) i nettleseren.

I VS Code: **Clone Repository** → lim inn `https://github.com/kmrodni-boop/spardame.git` → åpne mappen → `Terminal` → `npm install` → `npm run dev`.

Spillet lagrer språk, navn og parti i nettleseren (`localStorage`). Ingen konto, ingen database.

## Kommandoer

| Kommando | Hva den gjør |
| --- | --- |
| `npm run dev` | Utviklingsserver på port 8080 |
| `npm run build` | Produksjonsbygg |
| `npm run typecheck` | TypeScript-sjekk |
| `npm run test:game` | Regler og motor (19 tester) |

Port **8080** må være ledig. Stopp med `Ctrl+C`.

## Spill

- **Spardame:** spar dame 100, hjerter ess 20, øvrige hjerter 10, ruter knekt −100. Til 500.
- **Hearts:** hjerter 1, queen of spades 13. Shoot the moon gir 26 til de andre. Til 100.
- Første stikk: kløver 2. Følg farge. Hjerter kan ikke spilles ut før de er brutt.

## English

Browser card game: Norwegian Spardame and Windows Hearts. Four players, three AI opponents.

```bash
git clone https://github.com/kmrodni-boop/spardame.git
cd spardame
npm install
npm run dev
```

Then open [http://localhost:8080](http://localhost:8080). Node.js 22 required.
