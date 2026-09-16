import type { Locale } from "./i18n.ts";
import type { Difficulty, GameState, VariantId } from "./types.ts";

const KEY = "spardame.v1";
const SAVE_VERSION = 1;

export type Settings = {
  variant: VariantId;
  difficulty: Difficulty;
  playerName: string;
  aiNames: [string, string, string];
  muted: boolean;
  locale: Locale;
};

export type Stats = {
  gamesPlayed: number;
  gamesWon: number;
};

export type SaveBlob = {
  version: number;
  settings: Settings;
  stats: Stats;
  game: GameState | null;
};

export const DEFAULT_SETTINGS: Settings = {
  variant: "spardame",
  difficulty: "normal",
  playerName: "",
  aiNames: ["Kari", "Per", "Liv"],
  muted: false,
  locale: "en",
};

export const DEFAULT_STATS: Stats = { gamesPlayed: 0, gamesWon: 0 };

function defaults(): SaveBlob {
  return {
    version: SAVE_VERSION,
    settings: { ...DEFAULT_SETTINGS },
    stats: { ...DEFAULT_STATS },
    game: null,
  };
}

export function loadSave(): SaveBlob {
  if (typeof window === "undefined") return defaults();
  try {
    const raw = window.localStorage.getItem(KEY);
    if (!raw) return defaults();
    const parsed = JSON.parse(raw) as Partial<SaveBlob>;
    const settings: Settings = { ...DEFAULT_SETTINGS, ...parsed.settings };
    if (parsed.settings?.locale == null && settings.playerName === "Du") {
      settings.playerName = "";
      settings.locale = "en";
    }
    return {
      version: SAVE_VERSION,
      settings,
      stats: { ...DEFAULT_STATS, ...parsed.stats },
      game: parsed.game ?? null,
    };
  } catch {
    return defaults();
  }
}

export function writeSave(blob: SaveBlob): void {
  if (typeof window === "undefined") return;
  try {
    window.localStorage.setItem(KEY, JSON.stringify({ ...blob, version: SAVE_VERSION }));
  } catch {
    /* private mode / quota */
  }
}
