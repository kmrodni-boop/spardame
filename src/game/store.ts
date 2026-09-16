import { create } from "zustand";
import { choosePassCards, choosePlay } from "./ai.ts";
import { unlockAudio } from "./audio.ts";
import {
  commitPass,
  continueGame,
  createGame,
  playCard,
  resolveTrick,
  setPassSelection,
} from "./engine.ts";
import { t } from "./i18n.ts";
import {
  DEFAULT_SETTINGS,
  DEFAULT_STATS,
  loadSave,
  writeSave,
  type Settings,
  type Stats,
} from "./storage.ts";
import type { Card, Difficulty, Flash, GameState, PlayerId, VariantId } from "./types.ts";
import { PLAYERS } from "./types.ts";
import { getVariant } from "./variants.ts";

export type Screen = "menu" | "table";

type GameStore = {
  hydrated: boolean;
  screen: Screen;
  settings: Settings;
  stats: Stats;
  state: GameState | null;
  selected: string[];
  flash: Flash | null;
  rulesOpen: boolean;
  scoreOpen: boolean;
  hydrate: () => void;
  persist: () => void;
  setSettings: (patch: Partial<Settings>) => void;
  newGame: (variant?: VariantId) => void;
  continueGame: () => void;
  abandon: () => void;
  toggleCard: (id: string) => void;
  confirmPass: () => void;
  play: (id: string) => boolean;
  playAi: () => void;
  settleTrick: () => void;
  nextHand: () => void;
  setRulesOpen: (open: boolean) => void;
  setScoreOpen: (open: boolean) => void;
  clearFlash: () => void;
  playerName: (id: PlayerId) => string;
};

function fillAiPasses(state: GameState, difficulty: Difficulty): GameState {
  if (state.phase !== "passing") return state;
  let next = state;
  for (const p of PLAYERS) {
    if (p === 0) continue;
    const cards = choosePassCards(next, p, difficulty);
    next = setPassSelection(next, p, cards);
  }
  return next;
}

export const useGame = create<GameStore>((set, get) => ({
  hydrated: false,
  screen: "menu",
  settings: DEFAULT_SETTINGS,
  stats: DEFAULT_STATS,
  state: null,
  selected: [],
  flash: null,
  rulesOpen: false,
  scoreOpen: false,

  hydrate: () => {
    if (get().hydrated) return;
    const blob = loadSave();
    set({
      hydrated: true,
      settings: blob.settings,
      stats: blob.stats,
      state: blob.game,
    });
  },

  persist: () => {
    const { settings, stats, state, screen } = get();
    writeSave({
      version: 1,
      settings,
      stats,
      game: screen === "table" ? state : state?.phase === "gameOver" ? null : state,
    });
  },

  setSettings: (patch) => {
    set({ settings: { ...get().settings, ...patch } });
    get().persist();
  },

  newGame: (variant) => {
    unlockAudio();
    const v = variant ?? get().settings.variant;
    if (variant && variant !== get().settings.variant) {
      set({ settings: { ...get().settings, variant } });
    }
    const raw = createGame(v);
    const state = fillAiPasses(raw, get().settings.difficulty);
    set({
      screen: "table",
      state,
      selected: [],
      flash: null,
      scoreOpen: false,
    });
    get().persist();
  },

  continueGame: () => {
    unlockAudio();
    if (!get().state) return;
    set({ screen: "table", scoreOpen: get().state?.phase === "handEnd" || get().state?.phase === "gameOver" });
  },

  abandon: () => {
    set({ screen: "menu", selected: [], flash: null, scoreOpen: false });
    get().persist();
  },

  toggleCard: (id) => {
    const { state, selected } = get();
    if (!state || state.phase !== "passing") return;
    const v = getVariant(state.variant);
    const has = selected.includes(id);
    if (has) {
      set({ selected: selected.filter((x) => x !== id), flash: null });
      return;
    }
    if (selected.length >= v.passCount) return;
    const card = state.hands[0]!.find((c) => c.id === id);
    if (!card) return;
    if (!v.canPassQueen && card.id === "SQ") {
      set({ flash: { kind: "cannotPassQueen" } });
      return;
    }
    set({ selected: [...selected, id], flash: null });
  },

  confirmPass: () => {
    const { state, selected, settings } = get();
    if (!state || state.phase !== "passing") return;
    const v = getVariant(state.variant);
    if (selected.length !== v.passCount) {
      set({ flash: { kind: "pickCards", n: v.passCount } });
      return;
    }
    const cards = selected
      .map((id) => state.hands[0]!.find((c) => c.id === id))
      .filter((c): c is Card => Boolean(c));
    try {
      let next = setPassSelection(state, 0, cards);
      next = commitPass(next);
      const from = passFromName(next, settings);
      set({
        state: next,
        selected: [],
        flash: from ? { kind: "gotCards", n: v.passCount, from } : null,
      });
      get().persist();
    } catch {
      set({ flash: { kind: "passFailed" } });
    }
  },

  play: (id) => {
    const { state } = get();
    if (!state || state.phase !== "playing" || state.turn !== 0) return false;
    try {
      const next = playCard(state, 0, id);
      set({ state: next, selected: [], flash: null });
      get().persist();
      return true;
    } catch {
      set({ flash: { kind: "illegal" } });
      return false;
    }
  },

  playAi: () => {
    const { state, settings } = get();
    if (!state) return;
    if (state.phase !== "playing") return;
    if (state.turn === 0) return;
    const card = choosePlay(state, state.turn, settings.difficulty);
    const next = playCard(state, state.turn, card.id);
    set({ state: next, flash: null });
    get().persist();
  },

  settleTrick: () => {
    const { state, stats } = get();
    if (!state || state.phase !== "trickEnd") return;
    const next = resolveTrick(state);
    let nextStats = stats;
    if (next.phase === "gameOver") {
      const won = next.tied.includes(0);
      nextStats = {
        gamesPlayed: stats.gamesPlayed + 1,
        gamesWon: stats.gamesWon + (won ? 1 : 0),
      };
    }
    set({
      state: next,
      stats: nextStats,
      scoreOpen: next.phase === "handEnd" || next.phase === "gameOver",
    });
    get().persist();
  },

  nextHand: () => {
    const { state } = get();
    if (!state || state.phase !== "handEnd") return;
    const raw = continueGame(state);
    const filled = fillAiPasses(raw, get().settings.difficulty);
    set({ state: filled, selected: [], flash: null, scoreOpen: false });
    get().persist();
  },

  setRulesOpen: (open) => set({ rulesOpen: open }),
  setScoreOpen: (open) => set({ scoreOpen: open }),
  clearFlash: () => set({ flash: null }),

  playerName: (id) => seatName(id, get().settings),
}));

function passFromName(state: GameState, settings: Settings): string | null {
  if (state.passDir === "hold") return null;
  if (state.passDir === "left") return settings.aiNames[2];
  if (state.passDir === "right") return settings.aiNames[0];
  return settings.aiNames[1];
}

export function seatName(id: PlayerId, settings: Settings): string {
  if (id === 0) return settings.playerName.trim() || t(settings.locale).you;
  return settings.aiNames[id - 1] ?? t(settings.locale).opponents;
}
