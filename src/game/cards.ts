import type { Locale } from "./i18n.ts";
import { t } from "./i18n.ts";
import type { Card, Rank, Suit } from "./types.ts";

export const SUITS: Suit[] = ["clubs", "diamonds", "spades", "hearts"];
export const RANKS: Rank[] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

export const SUIT_ORDER: Record<Suit, number> = {
  clubs: 0,
  diamonds: 1,
  spades: 2,
  hearts: 3,
};

const SUIT_CHAR: Record<Suit, string> = {
  clubs: "C",
  diamonds: "D",
  spades: "S",
  hearts: "H",
};

const CHAR_SUIT: Record<string, Suit> = {
  C: "clubs",
  D: "diamonds",
  S: "spades",
  H: "hearts",
};

export function rankCode(rank: Rank): string {
  if (rank === 11) return "J";
  if (rank === 12) return "Q";
  if (rank === 13) return "K";
  if (rank === 14) return "A";
  return String(rank);
}

export function cardId(suit: Suit, rank: Rank): string {
  return `${SUIT_CHAR[suit]}${rankCode(rank)}`;
}

export function parseCard(id: string): Card {
  const suit = CHAR_SUIT[id[0] ?? ""];
  const rankStr = id.slice(1);
  const named: Record<string, Rank> = { J: 11, Q: 12, K: 13, A: 14 };
  const rank = named[rankStr] ?? (Number(rankStr) as Rank);
  if (!suit || !rank) throw new Error(`Invalid card: ${id}`);
  return { id, suit, rank };
}

export function makeCard(suit: Suit, rank: Rank): Card {
  return { id: cardId(suit, rank), suit, rank };
}

export function makeDeck(): Card[] {
  const deck: Card[] = [];
  for (const suit of SUITS) {
    for (const rank of RANKS) {
      deck.push(makeCard(suit, rank));
    }
  }
  return deck;
}

export function createRng(seed?: number): () => number {
  if (seed == null) return Math.random;
  let s = seed >>> 0;
  return () => {
    s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
    return s / 4294967296;
  };
}

export function shuffle<T>(items: T[], rng: () => number = Math.random): T[] {
  const next = items.slice();
  for (let i = next.length - 1; i > 0; i--) {
    const j = Math.floor(rng() * (i + 1));
    const a = next[i]!;
    next[i] = next[j]!;
    next[j] = a;
  }
  return next;
}

export function sortHand(hand: Card[]): Card[] {
  return hand.slice().sort((a, b) => {
    const suitDiff = SUIT_ORDER[a.suit] - SUIT_ORDER[b.suit];
    if (suitDiff !== 0) return suitDiff;
    return a.rank - b.rank;
  });
}

export function isQueenOfSpades(card: Card): boolean {
  return card.id === "SQ";
}

export function isJackOfDiamonds(card: Card): boolean {
  return card.id === "DJ";
}

export function isTwoOfClubs(card: Card): boolean {
  return card.id === "C2";
}

export function isHeart(card: Card): boolean {
  return card.suit === "hearts";
}

export function isPenaltyCard(card: Card): boolean {
  return isHeart(card) || isQueenOfSpades(card);
}

export function cardLabel(card: Card, locale: Locale = "en"): string {
  const copy = t(locale);
  if (isQueenOfSpades(card)) return copy.queenOfSpades;
  if (isJackOfDiamonds(card)) return copy.jackOfDiamonds;
  const rank =
    card.rank === 14
      ? copy.rank.ace
      : card.rank === 13
        ? copy.rank.king
        : card.rank === 12
          ? copy.rank.queen
          : card.rank === 11
            ? copy.rank.jack
            : String(card.rank);
  return `${copy.suitCap[card.suit]} ${rank}`;
}

export const cardLabelNb = (card: Card) => cardLabel(card, "nb");

export function sameCard(a: Card, b: Card): boolean {
  return a.id === b.id;
}

export function findCard(hand: Card[], id: string): Card | undefined {
  return hand.find((c) => c.id === id);
}

export function withoutCards(hand: Card[], ids: string[]): Card[] {
  const drop = new Set(ids);
  return hand.filter((c) => !drop.has(c.id));
}
