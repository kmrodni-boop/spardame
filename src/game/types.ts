export type Suit = "clubs" | "diamonds" | "spades" | "hearts";
export type Rank = 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14;

export type Card = {
  id: string;
  suit: Suit;
  rank: Rank;
};

export const PLAYERS = [0, 1, 2, 3] as const;
export type PlayerId = (typeof PLAYERS)[number];

export type PassDir = "left" | "right" | "across" | "hold";

export type Phase = "passing" | "playing" | "trickEnd" | "handEnd" | "gameOver";

export type VariantId = "spardame" | "hjerter";

export type Difficulty = "easy" | "normal" | "hard";

export type Flash =
  | { kind: "cannotPassQueen" }
  | { kind: "pickCards"; n: number }
  | { kind: "gotCards"; n: number; from: string }
  | { kind: "illegal" }
  | { kind: "passFailed" };

export type Play = {
  player: PlayerId;
  card: Card;
};

export type TrickRecord = {
  plays: Play[];
  winner: PlayerId;
};

export type HandScore = {
  raw: number[];
  applied: number[];
  moon: PlayerId | null;
  jackHolder: PlayerId | null;
};

export type GameState = {
  variant: VariantId;
  phase: Phase;
  hands: Card[][];
  taken: Card[][];
  trick: Play[];
  turn: PlayerId;
  heartsBroken: boolean;
  trickNumber: number;
  handNumber: number;
  scores: number[];
  handScore: HandScore | null;
  passDir: PassDir;
  passSelections: (Card[] | null)[];
  receivedPass: Card[];
  history: TrickRecord[];
  winner: PlayerId | null;
  tied: PlayerId[];
};

export type Variant = {
  id: VariantId;
  name: string;
  shortName: string;
  blurb: string;
  queenSpades: number;
  aceHearts: number;
  otherHearts: number;
  jackDiamonds: number;
  passCount: number;
  gameLimit: number;
  canPassQueen: boolean;
  moonOthers: number;
  rankLetters: { jack: string; queen: string; king: string; ace: string };
};
