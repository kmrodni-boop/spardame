import {
  findCard,
  isHeart,
  isJackOfDiamonds,
  isPenaltyCard,
  isQueenOfSpades,
  isTwoOfClubs,
  makeDeck,
  shuffle,
  sortHand,
  withoutCards,
} from "./cards.ts";
import type {
  Card,
  GameState,
  HandScore,
  PassDir,
  PlayerId,
  Play,
  VariantId,
} from "./types.ts";
import { PLAYERS } from "./types.ts";
import { getVariant, PASS_CYCLE, passOffset } from "./variants.ts";

function emptyTaken(): Card[][] {
  return [[], [], [], []];
}

function emptyPass(): (Card[] | null)[] {
  return [null, null, null, null];
}

export function cardPoints(card: Card, variantId: VariantId): number {
  const v = getVariant(variantId);
  if (isQueenOfSpades(card)) return v.queenSpades;
  if (isJackOfDiamonds(card)) return v.jackDiamonds;
  if (isHeart(card)) return card.rank === 14 ? v.aceHearts : v.otherHearts;
  return 0;
}

export function dealHands(rng: () => number = Math.random): Card[][] {
  const deck = shuffle(makeDeck(), rng);
  const hands: Card[][] = [[], [], [], []];
  for (let i = 0; i < deck.length; i++) {
    hands[i % 4]!.push(deck[i]!);
  }
  return hands.map(sortHand);
}

export function holderOf(hands: Card[][], id: string): PlayerId {
  for (const p of PLAYERS) {
    if (hands[p]!.some((c) => c.id === id)) return p;
  }
  throw new Error(`Kortet ${id} er ikke på noen hånd`);
}

export function nextPlayer(p: PlayerId): PlayerId {
  return ((p + 1) % 4) as PlayerId;
}

export function createGame(variant: VariantId, rng: () => number = Math.random): GameState {
  return startHand(variant, 0, [0, 0, 0, 0], rng);
}

export function startHand(
  variant: VariantId,
  handNumber: number,
  scores: number[],
  rng: () => number = Math.random,
): GameState {
  const hands = dealHands(rng);
  const passDir = PASS_CYCLE[handNumber % 4]!;
  const lead = holderOf(hands, "C2");
  const passing = passDir !== "hold";
  return {
    variant,
    phase: passing ? "passing" : "playing",
    hands,
    taken: emptyTaken(),
    trick: [],
    turn: lead,
    heartsBroken: false,
    trickNumber: 0,
    handNumber,
    scores: scores.slice() as number[],
    handScore: null,
    passDir,
    passSelections: emptyPass(),
    receivedPass: [],
    history: [],
    winner: null,
    tied: [],
  };
}

export function passableCards(state: GameState, player: PlayerId): Card[] {
  const v = getVariant(state.variant);
  const hand = state.hands[player]!;
  if (v.canPassQueen) return hand;
  return hand.filter((c) => !isQueenOfSpades(c));
}

export function setPassSelection(state: GameState, player: PlayerId, cards: Card[]): GameState {
  const v = getVariant(state.variant);
  if (state.phase !== "passing") throw new Error("Ikke i byttefase");
  if (cards.length !== v.passCount) throw new Error(`Velg ${v.passCount} kort`);
  const hand = state.hands[player]!;
  for (const card of cards) {
    if (!findCard(hand, card.id)) throw new Error("Kortet er ikke på hånden");
    if (!v.canPassQueen && isQueenOfSpades(card)) {
      throw new Error("Spar dame kan ikke byttes bort");
    }
  }
  const ids = new Set(cards.map((c) => c.id));
  if (ids.size !== cards.length) throw new Error("Duplikat i byttet");
  const passSelections = state.passSelections.slice() as (Card[] | null)[];
  passSelections[player] = cards;
  return { ...state, passSelections };
}

export function commitPass(state: GameState): GameState {
  const v = getVariant(state.variant);
  if (state.phase !== "passing") return state;
  if (state.passDir === "hold") {
    return { ...state, phase: "playing", turn: holderOf(state.hands, "C2") };
  }
  if (state.passSelections.some((s) => !s || s.length !== v.passCount)) {
    throw new Error("Alle spillere må velge kort før byttet");
  }
  const offset = passOffset(state.passDir);
  const nextHands = state.hands.map((h) => h.slice()) as Card[][];
  const received: Card[][] = [[], [], [], []];
  for (const p of PLAYERS) {
    const giving = state.passSelections[p]!;
    nextHands[p] = withoutCards(nextHands[p]!, giving.map((c) => c.id));
    const dest = ((p + offset) % 4) as PlayerId;
    received[dest] = giving;
  }
  for (const p of PLAYERS) {
    nextHands[p] = sortHand([...nextHands[p]!, ...received[p]!]);
  }
  return {
    ...state,
    hands: nextHands,
    receivedPass: received[0]!,
    passSelections: emptyPass(),
    phase: "playing",
    turn: holderOf(nextHands, "C2"),
  };
}

export function legalMoves(state: GameState, player: PlayerId): Card[] {
  if (state.phase !== "playing") return [];
  if (state.turn !== player) return [];
  const hand = state.hands[player]!;
  if (hand.length === 0) return [];

  if (state.trick.length === 0) {
    if (state.trickNumber === 0) {
      const two = hand.find(isTwoOfClubs);
      if (two) return [two];
    }
    if (!state.heartsBroken) {
      const nonHearts = hand.filter((c) => !isHeart(c));
      if (nonHearts.length > 0) return nonHearts;
    }
    return hand.slice();
  }

  const leadSuit = state.trick[0]!.card.suit;
  const matching = hand.filter((c) => c.suit === leadSuit);
  if (matching.length > 0) return matching;

  if (state.trickNumber === 0) {
    const safe = hand.filter((c) => !isPenaltyCard(c));
    if (safe.length > 0) return safe;
  }
  return hand.slice();
}

export function isLegalPlay(state: GameState, player: PlayerId, cardId: string): boolean {
  return legalMoves(state, player).some((c) => c.id === cardId);
}

export function trickWinner(plays: Play[]): PlayerId {
  const leadSuit = plays[0]!.card.suit;
  let best = plays[0]!;
  for (let i = 1; i < plays.length; i++) {
    const play = plays[i]!;
    if (play.card.suit === leadSuit && play.card.rank > best.card.rank) {
      best = play;
    }
  }
  return best.player;
}

export function playCard(state: GameState, player: PlayerId, cardId: string): GameState {
  if (!isLegalPlay(state, player, cardId)) {
    throw new Error("Ulovlig trekk");
  }
  const card = findCard(state.hands[player]!, cardId);
  if (!card) throw new Error("Kortet er ikke på hånden");
  const hands = state.hands.map((h, i) =>
    i === player ? h.filter((c) => c.id !== cardId) : h,
  ) as Card[][];
  const trick = [...state.trick, { player, card }];
  const broken = state.heartsBroken || isHeart(card);
  if (trick.length < 4) {
    return {
      ...state,
      hands,
      trick,
      turn: nextPlayer(player),
      heartsBroken: broken,
    };
  }
  return {
    ...state,
    hands,
    trick,
    heartsBroken: broken,
    phase: "trickEnd",
  };
}

export function takenPoints(taken: Card[], variantId: VariantId): number {
  return taken.reduce((sum, card) => sum + cardPoints(card, variantId), 0);
}

export function hasShotTheMoon(taken: Card[]): boolean {
  const hearts = taken.filter(isHeart).length;
  const queen = taken.some(isQueenOfSpades);
  return hearts === 13 && queen;
}

function scoreFromTaken(taken: Card[][], variantId: VariantId): HandScore {
  const raw = taken.map((pile) => takenPoints(pile, variantId));
  const moon = PLAYERS.find((p) => hasShotTheMoon(taken[p]!)) ?? null;
  const jackHolder = PLAYERS.find((p) => taken[p]!.some(isJackOfDiamonds)) ?? null;
  const applied = raw.slice();
  if (moon != null) {
    const v = getVariant(variantId);
    for (const p of PLAYERS) {
      if (p === moon) {
        const jack = v.jackDiamonds !== 0 && jackHolder === moon ? v.jackDiamonds : 0;
        applied[p] = jack;
      } else {
        applied[p] = v.moonOthers;
      }
    }
  }
  return { raw, applied, moon, jackHolder };
}

export function resolveTrick(state: GameState): GameState {
  if (state.phase !== "trickEnd" || state.trick.length !== 4) return state;
  const winner = trickWinner(state.trick);
  const taken = state.taken.map((pile, i) =>
    i === winner ? [...pile, ...state.trick.map((p) => p.card)] : pile,
  ) as Card[][];
  const history = [...state.history, { plays: state.trick, winner }];
  const lastTrick = state.trickNumber >= 12;
  if (!lastTrick) {
    return {
      ...state,
      taken,
      history,
      trick: [],
      turn: winner,
      trickNumber: state.trickNumber + 1,
      phase: "playing",
    };
  }
  const handScore = scoreFromTaken(taken, state.variant);
  const scores = state.scores.map((s, i) => s + handScore.applied[i]!) as number[];
  const v = getVariant(state.variant);
  const over = scores.some((s) => s >= v.gameLimit);
  if (!over) {
    return {
      ...state,
      taken,
      history,
      trick: [],
      trickNumber: state.trickNumber + 1,
      scores,
      handScore,
      phase: "handEnd",
    };
  }
  const min = Math.min(...scores);
  const tied = PLAYERS.filter((p) => scores[p] === min);
  return {
    ...state,
    taken,
    history,
    trick: [],
    trickNumber: state.trickNumber + 1,
    scores,
    handScore,
    phase: "gameOver",
    winner: tied[0]!,
    tied: [...tied],
  };
}

export function continueGame(state: GameState, rng: () => number = Math.random): GameState {
  if (state.phase !== "handEnd") return state;
  return startHand(state.variant, state.handNumber + 1, state.scores, rng);
}

export function currentWinnerOfTrick(trick: Play[]): PlayerId | null {
  if (trick.length === 0) return null;
  return trickWinner(trick);
}

export function suitLed(state: GameState): Card["suit"] | null {
  return state.trick[0]?.card.suit ?? null;
}

export function queenStillOut(state: GameState): boolean {
  if (state.trick.some((p) => isQueenOfSpades(p.card))) return false;
  return !state.taken.some((pile) => pile.some(isQueenOfSpades));
}

export function jackStillOut(state: GameState): boolean {
  if (state.trick.some((p) => isJackOfDiamonds(p.card))) return false;
  return !state.taken.some((pile) => pile.some(isJackOfDiamonds));
}

export function pointCardsTaken(pile: Card[], variantId: VariantId): Card[] {
  return pile.filter((c) => cardPoints(c, variantId) !== 0);
}

export function cloneState(state: GameState): GameState {
  return {
    ...state,
    hands: state.hands.map((h) => h.slice()),
    taken: state.taken.map((h) => h.slice()),
    trick: state.trick.slice(),
    scores: state.scores.slice(),
    passSelections: state.passSelections.map((s) => (s ? s.slice() : null)),
    receivedPass: state.receivedPass.slice(),
    history: state.history.map((t) => ({ plays: t.plays.slice(), winner: t.winner })),
    tied: state.tied.slice(),
    handScore: state.handScore
      ? {
          ...state.handScore,
          raw: state.handScore.raw.slice(),
          applied: state.handScore.applied.slice(),
        }
      : null,
  };
}

export function remainingCards(state: GameState, viewer: PlayerId): Card[] {
  const seen = new Set<string>();
  for (const card of state.hands[viewer]!) seen.add(card.id);
  for (const pile of state.taken) for (const card of pile) seen.add(card.id);
  for (const play of state.trick) seen.add(play.card.id);
  return makeDeck().filter((c) => !seen.has(c.id));
}

export function knownVoids(state: GameState): boolean[][] {
  const voids = PLAYERS.map(() => ({
    clubs: false,
    diamonds: false,
    spades: false,
    hearts: false,
  }));
  for (const trick of state.history) {
    const lead = trick.plays[0]!.card.suit;
    for (const play of trick.plays) {
      if (play.card.suit !== lead) voids[play.player]![lead] = true;
    }
  }
  if (state.trick.length > 0) {
    const lead = state.trick[0]!.card.suit;
    for (const play of state.trick) {
      if (play.card.suit !== lead) voids[play.player]![lead] = true;
    }
  }
  return voids.map((row) => [row.clubs, row.diamonds, row.spades, row.hearts]);
}

export function passRecipient(from: PlayerId, dir: PassDir): PlayerId {
  return ((from + passOffset(dir)) % 4) as PlayerId;
}
