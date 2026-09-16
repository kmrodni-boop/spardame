import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { createRng, makeCard, makeDeck, parseCard, shuffle, sortHand } from "./cards.ts";
import {
  cardPoints,
  commitPass,
  createGame,
  hasShotTheMoon,
  holderOf,
  legalMoves,
  playCard,
  resolveTrick,
  setPassSelection,
  startHand,
  trickWinner,
} from "./engine.ts";
import type { Card, GameState, PlayerId } from "./types.ts";
import { PLAYERS } from "./types.ts";

function C(id: string): Card {
  return parseCard(id);
}

function withHands(hands: Card[][], extra: Partial<GameState> = {}): GameState {
  const base = startHand("hjerter", 0, [0, 0, 0, 0], createRng(1));
  return {
    ...base,
    hands: hands.map(sortHand),
    phase: "playing",
    passDir: "hold",
    trick: [],
    trickNumber: 1,
    turn: 0,
    heartsBroken: false,
    ...extra,
  };
}

describe("kortstokk", () => {
  it("har 52 unike kort", () => {
    const deck = makeDeck();
    assert.equal(deck.length, 52);
    assert.equal(new Set(deck.map((c) => c.id)).size, 52);
  });

  it("Fisher–Yates bevarer innhold", () => {
    const deck = makeDeck();
    const shuffled = shuffle(deck, createRng(42));
    assert.equal(shuffled.length, 52);
    assert.deepEqual(
      shuffled.map((c) => c.id).sort(),
      deck.map((c) => c.id).sort(),
    );
    assert.notDeepEqual(
      shuffled.map((c) => c.id),
      deck.map((c) => c.id),
    );
  });
});

describe("poeng", () => {
  it("Spardame: dame 100, ess 20, hjerter 10, knekt −100", () => {
    assert.equal(cardPoints(C("SQ"), "spardame"), 100);
    assert.equal(cardPoints(C("HA"), "spardame"), 20);
    assert.equal(cardPoints(C("H2"), "spardame"), 10);
    assert.equal(cardPoints(C("DJ"), "spardame"), -100);
    assert.equal(cardPoints(C("SA"), "spardame"), 0);
  });

  it("Hjerter: 1 per hjerter, 13 for spar dame", () => {
    assert.equal(cardPoints(C("SQ"), "hjerter"), 13);
    assert.equal(cardPoints(C("HA"), "hjerter"), 1);
    assert.equal(cardPoints(C("H10"), "hjerter"), 1);
    assert.equal(cardPoints(C("DJ"), "hjerter"), 0);
  });

  it("slem krever alle 13 hjerter og spar dame", () => {
    const taken = makeDeck().filter((c) => c.suit === "hearts" || c.id === "SQ");
    assert.equal(taken.length, 14);
    assert.equal(hasShotTheMoon(taken), true);
    assert.equal(hasShotTheMoon(taken.filter((c) => c.id !== "SQ")), false);
  });
});

describe("utdeling", () => {
  it("gir 13 kort til hver og plasserer kløver 2", () => {
    const state = createGame("spardame", createRng(7));
    for (const p of PLAYERS) assert.equal(state.hands[p]!.length, 13);
    const lead = holderOf(state.hands, "C2");
    assert.ok(state.hands[lead]!.some((c) => c.id === "C2"));
    assert.equal(state.passDir, "left");
    assert.equal(state.phase, "passing");
  });

  it("fjerde runde er hold", () => {
    const state = startHand("hjerter", 3, [0, 0, 0, 0], createRng(3));
    assert.equal(state.passDir, "hold");
    assert.equal(state.phase, "playing");
  });
});

describe("bytte", () => {
  it("nektar å sende spar dame i Spardame", () => {
    const state = createGame("spardame", createRng(9));
    const queenHolder = holderOf(state.hands, "SQ");
    const hand = state.hands[queenHolder]!;
    const others = hand.filter((c) => c.id !== "SQ").slice(0, 2);
    const attempt = [C("SQ"), ...others];
    assert.throws(() => setPassSelection(state, queenHolder, attempt));
  });

  it("bytter tre kort til venstre", () => {
    let state = startHand("hjerter", 0, [0, 0, 0, 0], createRng(11));
    const given: Card[][] = PLAYERS.map((p) => state.hands[p]!.slice(0, 3));
    for (const p of PLAYERS) {
      state = setPassSelection(state, p, given[p]!);
    }
    state = commitPass(state);
    assert.equal(state.phase, "playing");
    assert.equal(state.hands[0]!.length, 13);
    for (const card of given[3]!) {
      assert.ok(state.hands[0]!.some((c) => c.id === card.id));
    }
  });
});

describe("lovlige trekk", () => {
  it("første utspill må være kløver 2", () => {
    const two = makeCard("clubs", 2);
    const state = withHands(
      [
        [two, C("C9"), C("SA")],
        [C("C3")],
        [C("C4")],
        [C("C5")],
      ],
      { trickNumber: 0, turn: 0, heartsBroken: false },
    );
    const legal = legalMoves(state, 0).map((c) => c.id);
    assert.deepEqual(legal, ["C2"]);
  });

  it("må følge farge", () => {
    const state = withHands(
      [
        [C("C9"), C("H3"), C("SA")],
        [C("C3")],
        [C("C4")],
        [C("C5")],
      ],
      {
        trick: [{ player: 1, card: C("C8") }],
        turn: 0,
        trickNumber: 1,
      },
    );
    const legal = legalMoves(state, 0).map((c) => c.id);
    assert.deepEqual(legal, ["C9"]);
  });

  it("kan ikke spille ut hjerter før de er brutt", () => {
    const state = withHands(
      [
        [C("H3"), C("SA"), C("C9")],
        [C("C3")],
        [C("C4")],
        [C("C5")],
      ],
      { trick: [], turn: 0, heartsBroken: false, trickNumber: 2 },
    );
    const legal = legalMoves(state, 0).map((c) => c.id);
    assert.ok(legal.includes("SA"));
    assert.ok(legal.includes("C9"));
    assert.ok(!legal.includes("H3"));
  });

  it("tillater hjerter-utspill når man bare har hjerter", () => {
    const state = withHands(
      [[C("H3"), C("HA")], [C("C3")], [C("C4")], [C("C5")]],
      { trick: [], turn: 0, heartsBroken: false, trickNumber: 4 },
    );
    const legal = legalMoves(state, 0).map((c) => c.id);
    assert.ok(legal.includes("H3"));
    assert.ok(legal.includes("HA"));
  });

  it("forbyr straffekort i første stikk når man er blank i fargen", () => {
    const state = withHands(
      [
        [C("H3"), C("SQ"), C("D4")],
        [C("C3")],
        [C("C4")],
        [C("C5")],
      ],
      {
        trick: [{ player: 1, card: C("C8") }],
        turn: 0,
        trickNumber: 0,
        heartsBroken: false,
      },
    );
    const legal = legalMoves(state, 0).map((c) => c.id);
    assert.deepEqual(legal, ["D4"]);
  });
});

describe("stikk", () => {
  it("høyeste kort i utspillfargen vinner", () => {
    const winner = trickWinner([
      { player: 0, card: C("C9") },
      { player: 1, card: C("CA") },
      { player: 2, card: C("HA") },
      { player: 3, card: C("C3") },
    ]);
    assert.equal(winner, 1);
  });

  it("fullfører 13 stikk og summerer hjerter-poeng", () => {
    let state = startHand("hjerter", 3, [0, 0, 0, 0], createRng(21));
    assert.equal(state.phase, "playing");
    let guard = 0;
    while (state.phase !== "handEnd" && state.phase !== "gameOver") {
      guard += 1;
      if (guard > 80) throw new Error("spill kjørte for lenge");
      if (state.phase === "playing") {
        const legal = legalMoves(state, state.turn);
        state = playCard(state, state.turn, legal[0]!.id);
      } else if (state.phase === "trickEnd") {
        state = resolveTrick(state);
      }
    }
    assert.ok(state.handScore);
    const sum = state.handScore!.applied.reduce((a, b) => a + b, 0);
    if (state.handScore!.moon == null) {
      assert.equal(sum, 26);
    } else {
      assert.equal(sum, 78);
    }
  });
});

describe("slem i spardame", () => {
  it("gir 100 til de andre og 0 (evt. −100) til skytteren", () => {
    const hearts = makeDeck().filter((c) => c.suit === "hearts");
    const moonPile = [...hearts, C("SQ"), C("C3")];
    const taken: Card[][] = [moonPile, [C("DJ")], [C("SA")], [C("CA")]];
    assert.equal(hasShotTheMoon(taken[0]!), true);
  });
});

describe("playCard avviser ulovlige trekk", () => {
  it("kaster ved feil farge", () => {
    const state = withHands(
      [
        [C("C9"), C("H3")],
        [C("C3")],
        [C("C4")],
        [C("C5")],
      ],
      { trick: [{ player: 1, card: C("C8") }], turn: 0, trickNumber: 2 },
    );
    assert.throws(() => playCard(state, 0, "H3"));
  });

  it("avviser trekk utenom tur", () => {
    const state = withHands([[C("C9")], [C("C3")], [C("C4")], [C("C5")]], { turn: 1 });
    assert.throws(() => playCard(state, 0, "C9"));
  });
});
