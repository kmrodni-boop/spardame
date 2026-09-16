import { t, type Locale } from "./i18n.ts";
import type { PassDir, Variant, VariantId } from "./types.ts";

export type { Variant, VariantId };

export const VARIANTS: Record<VariantId, Variant> = {
  spardame: {
    id: "spardame",
    name: "Spardame",
    shortName: "Norsk",
    blurb: "Spar dame 100, hjerter ess 20, ruter knekt −100. Til 500 poeng.",
    queenSpades: 100,
    aceHearts: 20,
    otherHearts: 10,
    jackDiamonds: -100,
    passCount: 3,
    gameLimit: 500,
    canPassQueen: false,
    moonOthers: 100,
    rankLetters: { jack: "Kn", queen: "D", king: "K", ace: "A" },
  },
  hjerter: {
    id: "hjerter",
    name: "Hjerter",
    shortName: "Windows",
    blurb: "Hjerter 1, spar dame 13. Skyte månen gir 26 til de andre. Til 100 poeng.",
    queenSpades: 13,
    aceHearts: 1,
    otherHearts: 1,
    jackDiamonds: 0,
    passCount: 3,
    gameLimit: 100,
    canPassQueen: true,
    moonOthers: 26,
    rankLetters: { jack: "J", queen: "Q", king: "K", ace: "A" },
  },
};

export function getVariant(id: VariantId): Variant {
  return VARIANTS[id];
}

export const PASS_CYCLE: PassDir[] = ["left", "right", "across", "hold"];

export function passOffset(dir: PassDir): number {
  if (dir === "left") return 1;
  if (dir === "right") return 3;
  if (dir === "across") return 2;
  return 0;
}

export function passLabel(dir: PassDir, locale: Locale = "en"): string {
  return t(locale).pass[dir];
}
