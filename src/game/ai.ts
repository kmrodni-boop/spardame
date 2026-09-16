import {
  isHeart,
  isJackOfDiamonds,
  isQueenOfSpades,
  SUIT_ORDER,
} from "./cards.ts";
import {
  currentWinnerOfTrick,
  jackStillOut,
  legalMoves,
  queenStillOut,
  remainingCards,
} from "./engine.ts";
import type { Card, Difficulty, GameState, PlayerId, Suit } from "./types.ts";
import { getVariant } from "./variants.ts";

function suitCount(hand: Card[], suit: Suit): number {
  return hand.filter((c) => c.suit === suit).length;
}

function highest(cards: Card[]): Card | undefined {
  return cards.slice().sort((a, b) => b.rank - a.rank)[0];
}

function lowest(cards: Card[]): Card | undefined {
  return cards.slice().sort((a, b) => a.rank - b.rank)[0];
}

function byRankAsc(cards: Card[]): Card[] {
  return cards.slice().sort((a, b) => a.rank - b.rank);
}

function byRankDesc(cards: Card[]): Card[] {
  return cards.slice().sort((a, b) => b.rank - a.rank);
}

function noise(difficulty: Difficulty): number {
  if (difficulty === "easy") return (Math.random() - 0.5) * 80;
  if (difficulty === "normal") return (Math.random() - 0.5) * 12;
  return (Math.random() - 0.5) * 2;
}

export function choosePassCards(state: GameState, player: PlayerId, difficulty: Difficulty): Card[] {
  const v = getVariant(state.variant);
  const pool = (v.canPassQueen ? state.hands[player]! : state.hands[player]!.filter((c) => !isQueenOfSpades(c))).slice();
  const need = v.passCount;
  const scored = pool.map((card) => ({ card, score: passScore(card, state.hands[player]!, v.canPassQueen) + noise(difficulty) }));
  scored.sort((a, b) => b.score - a.score);

  const chosen: Card[] = [];
  const used = new Set<string>();

  const tryVoid = bestVoidCandidates(pool, state.hands[player]!);
  for (const card of tryVoid) {
    if (chosen.length >= need) break;
    if (used.has(card.id)) continue;
    if (!v.canPassQueen && isQueenOfSpades(card)) continue;
    chosen.push(card);
    used.add(card.id);
  }

  for (const row of scored) {
    if (chosen.length >= need) break;
    if (used.has(row.card.id)) continue;
    chosen.push(row.card);
    used.add(row.card.id);
  }
  return chosen.slice(0, need);
}

function passScore(card: Card, hand: Card[], canPassQueen: boolean): number {
  if (isJackOfDiamonds(card)) return -80;
  if (isQueenOfSpades(card)) return canPassQueen && suitCount(hand, "spades") <= 4 ? 100 : -20;
  if (card.suit === "hearts" && card.rank >= 12) return 70 + card.rank;
  if (card.suit === "spades" && card.rank >= 13) {
    return suitCount(hand, "spades") <= 3 ? 90 : 40;
  }
  if (card.rank >= 13) return 35 + card.rank;
  if (card.rank === 12) return 20;
  return card.rank * 0.3;
}

function bestVoidCandidates(pool: Card[], hand: Card[]): Card[] {
  const suits: Suit[] = ["clubs", "diamonds", "hearts"];
  let best: Card[] = [];
  for (const suit of suits) {
    const cards = pool.filter((c) => c.suit === suit);
    const total = suitCount(hand, suit);
    if (total > 0 && total <= 3 && cards.length === total) {
      if (best.length === 0 || cards.length < best.length) best = cards;
    }
  }
  return best;
}

function moonUrge(state: GameState, player: PlayerId): number {
  const taken = state.taken[player]!;
  const hearts = taken.filter(isHeart).length;
  const hasQ = taken.some(isQueenOfSpades);
  const pointsHeld = hearts + (hasQ ? 5 : 0);
  if (pointsHeld < 6) return 0;
  const hand = state.hands[player]!;
  const highHearts = hand.filter((c) => c.suit === "hearts" && c.rank >= 12).length;
  const remainingHearts = 13 - hearts - state.trick.filter((p) => isHeart(p.card)).length;
  if (hasQ && hearts >= 8) return 2;
  if (pointsHeld >= 9 && highHearts >= 1 && remainingHearts <= 6) return 1;
  return 0;
}

export function choosePlay(state: GameState, player: PlayerId, difficulty: Difficulty): Card {
  const legal = legalMoves(state, player);
  if (legal.length === 0) throw new Error("Ingen lovlige trekk");
  if (legal.length === 1) return legal[0]!;
  if (difficulty === "easy" && Math.random() < 0.45) {
    return legal[Math.floor(Math.random() * legal.length)]!;
  }

  const moon = moonUrge(state, player);
  const scored = legal.map((card) => ({
    card,
    score: evaluatePlay(state, player, card, moon, difficulty) + noise(difficulty),
  }));
  scored.sort((a, b) => b.score - a.score);
  return scored[0]!.card;
}

function evaluatePlay(
  state: GameState,
  player: PlayerId,
  card: Card,
  moon: number,
  difficulty: Difficulty,
): number {
  const trick = state.trick;
  const v = getVariant(state.variant);
  const queenOut = queenStillOut(state);
  const jackOut = v.jackDiamonds !== 0 && jackStillOut(state);
  const unseen = difficulty === "hard" ? remainingCards(state, player) : [];

  if (trick.length === 0) {
    return evalLead(state, player, card, moon, queenOut, jackOut, unseen);
  }

  const leadSuit = trick[0]!.card.suit;
  const following = card.suit === leadSuit;
  const current = currentWinnerOfTrick(trick);
  const wouldWin = wouldTake(trick, card, player);
  const pointsOnTrick = trick.reduce((s, p) => s + (isPenaltyCardValue(p.card, v.id) ? 1 : 0), 0);
  const lastToPlay = trick.length === 3;

  if (moon > 0) {
    if (wouldWin) return 50 + card.rank + (isHeart(card) ? 8 : 0) + (isQueenOfSpades(card) ? 20 : 0);
    return 10 - card.rank;
  }

  if (following) {
    if (leadSuit === "spades") {
      if (isQueenOfSpades(card)) {
        const maxSpade = Math.max(...trick.filter((p) => p.card.suit === "spades").map((p) => p.card.rank));
        if (maxSpade > 12) return 95;
        if (lastToPlay && current !== player) return 40;
        return -90;
      }
      if (wouldWin && queenOut && card.rank >= 13) return -70;
      if (!wouldWin) return 30 + card.rank;
      return 5 - card.rank;
    }

    if (leadSuit === "diamonds" && v.jackDiamonds !== 0) {
      if (isJackOfDiamonds(card)) {
        if (wouldWin) return 80;
        return -60;
      }
      if (wouldWin && jackOut && card.rank > 11) return 55;
      if (wouldWin && !jackOut) return 8 - card.rank;
    }

    if (leadSuit === "hearts") {
      if (!wouldWin) return 40 + card.rank;
      return -20 - card.rank;
    }

    if (!wouldWin) return 25 + card.rank * 0.4;
    if (pointsOnTrick > 0) return -30 - card.rank;
    if (lastToPlay) return 8 - card.rank * 0.2;
    return 5 - card.rank * 0.5;
  }

  // Void: dumping
  if (isQueenOfSpades(card)) return 120;
  if (isJackOfDiamonds(card)) return -100;
  if (isHeart(card) && card.rank >= 12) return 70 + card.rank;
  if (isHeart(card)) return 40 + card.rank;
  if (card.suit === "spades" && card.rank >= 13 && queenOut) return 85;
  if (card.rank >= 13) return 45 + card.rank;
  if (card.rank >= 11) return 20 + card.rank;
  return card.rank;
}

function evalLead(
  state: GameState,
  player: PlayerId,
  card: Card,
  moon: number,
  queenOut: boolean,
  jackOut: boolean,
  unseen: Card[],
): number {
  const v = getVariant(state.variant);
  const hand = state.hands[player]!;
  if (moon > 0) {
    if (isHeart(card) || isQueenOfSpades(card)) return 60 + card.rank;
    return 20 + card.rank;
  }
  if (isQueenOfSpades(card)) return -100;
  if (isHeart(card)) return -40 - card.rank;
  if (card.suit === "spades" && queenOut && card.rank >= 13) return -80;
  if (card.suit === "spades" && queenOut) {
    const mySpades = hand.filter((c) => c.suit === "spades" && c.rank < 12);
    if (mySpades.length >= 3 && card.rank <= 6) return 18;
    return -15;
  }
  if (isJackOfDiamonds(card)) return jackOut ? 10 : -20;

  const count = suitCount(hand, card.suit);
  let score = 20 - card.rank + (count >= 4 ? 6 : 0);
  if (card.suit === "clubs" || card.suit === "diamonds") score += 8;
  if (unseen.length && difficultySafe(unseen, card)) score += 4;
  if (v.jackDiamonds !== 0 && card.suit === "diamonds" && card.rank < 11 && jackOut) {
    score += 12;
  }
  return score;
}

function difficultySafe(unseen: Card[], card: Card): boolean {
  const higher = unseen.filter((c) => c.suit === card.suit && c.rank > card.rank);
  return higher.length >= 2;
}

function wouldTake(trick: { player: PlayerId; card: Card }[], card: Card, player: PlayerId): boolean {
  const plays = [...trick, { player, card }];
  const leadSuit = plays[0]!.card.suit;
  let best = plays[0]!;
  for (const play of plays) {
    if (play.card.suit === leadSuit && play.card.rank > best.card.rank) best = play;
  }
  return best.player === player;
}

function isPenaltyCardValue(card: Card, variantId: GameState["variant"]): boolean {
  if (isHeart(card) || isQueenOfSpades(card)) return true;
  if (variantId === "spardame" && isJackOfDiamonds(card)) return false;
  return false;
}

export function thinkDelayMs(difficulty: Difficulty, legalCount: number): number {
  const base = difficulty === "easy" ? 520 : difficulty === "normal" ? 680 : 860;
  const extra = legalCount > 6 ? 180 : 90;
  return base + Math.floor(Math.random() * extra);
}

export { SUIT_ORDER };
