import { useEffect } from "react";
import { thinkDelayMs } from "@/game/ai";
import { legalMoves } from "@/game/engine";
import {
  resumeAudio,
  sfxCard,
  sfxIllegal,
  sfxMoon,
  sfxPass,
  sfxTrick,
  unlockAudio,
} from "@/game/audio";
import { htmlLang } from "@/game/i18n";
import { seatName, useGame } from "@/game/store";
import { PLAYERS } from "@/game/types";
import { GameTable } from "./GameTable";
import { MenuScreen } from "./MenuScreen";
import { RulesPanel } from "./RulesPanel";
import { ScoreSheet } from "./ScoreSheet";

const TRICK_SETTLE_MS = 1450;
const TRICK_SETTLE_REDUCED_MS = 420;

export function GameApp() {
  const hydrated = useGame((s) => s.hydrated);
  const hydrate = useGame((s) => s.hydrate);
  const screen = useGame((s) => s.screen);
  const state = useGame((s) => s.state);
  const settings = useGame((s) => s.settings);
  const stats = useGame((s) => s.stats);
  const selected = useGame((s) => s.selected);
  const flash = useGame((s) => s.flash);
  const rulesOpen = useGame((s) => s.rulesOpen);
  const scoreOpen = useGame((s) => s.scoreOpen);

  useEffect(() => {
    hydrate();
  }, [hydrate]);

  useEffect(() => {
    document.documentElement.lang = htmlLang(settings.locale);
  }, [settings.locale]);

  useEffect(() => {
    const onVis = () => {
      if (document.visibilityState === "visible") resumeAudio();
      useGame.getState().persist();
    };
    document.addEventListener("visibilitychange", onVis);
    window.addEventListener("pagehide", onVis);
    return () => {
      document.removeEventListener("visibilitychange", onVis);
      window.removeEventListener("pagehide", onVis);
    };
  }, []);

  useEffect(() => {
    if (!flash) return;
    const timer = window.setTimeout(() => useGame.getState().clearFlash(), 3200);
    return () => window.clearTimeout(timer);
  }, [flash]);

  useEffect(() => {
    if (!state || screen !== "table") return;
    if (state.phase !== "playing") return;
    if (state.turn === 0) return;
    const legal = legalMoves(state, state.turn);
    const delay = thinkDelayMs(settings.difficulty, legal.length);
    const timer = window.setTimeout(() => {
      if (!settings.muted) sfxCard();
      useGame.getState().playAi();
    }, delay);
    return () => window.clearTimeout(timer);
  }, [
    screen,
    state?.phase,
    state?.turn,
    state?.trickNumber,
    state?.trick.length,
    settings.difficulty,
    settings.muted,
  ]);

  useEffect(() => {
    if (!state || screen !== "table") return;
    if (state.phase !== "trickEnd") return;
    if (!settings.muted) sfxTrick();
    const reduced =
      typeof window !== "undefined" && window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    const timer = window.setTimeout(
      () => {
        useGame.getState().settleTrick();
      },
      reduced ? TRICK_SETTLE_REDUCED_MS : TRICK_SETTLE_MS,
    );
    return () => window.clearTimeout(timer);
  }, [screen, state?.phase, state?.trickNumber, settings.muted]);

  useEffect(() => {
    if (state?.handScore?.moon == null) return;
    if (state.phase !== "handEnd" && state.phase !== "gameOver") return;
    if (!settings.muted) sfxMoon();
  }, [state?.phase, state?.handNumber, state?.handScore?.moon, settings.muted]);

  const names = PLAYERS.map((p) => seatName(p, settings));
  const canContinue = Boolean(hydrated && state && state.phase !== "gameOver");

  return (
    <>
      {screen === "menu" || !state ? (
        <MenuScreen
          settings={settings}
          stats={stats}
          canContinue={canContinue && screen === "menu"}
          onSettings={(patch) => useGame.getState().setSettings(patch)}
          onStart={(variant) => useGame.getState().newGame(variant)}
          onContinue={() => useGame.getState().continueGame()}
          onRules={() => useGame.getState().setRulesOpen(true)}
        />
      ) : (
        <GameTable
          state={state}
          names={names}
          selected={selected}
          flash={flash}
          muted={settings.muted}
          locale={settings.locale}
          onToggleMute={() => {
            unlockAudio();
            useGame.getState().setSettings({ muted: !settings.muted });
          }}
          onRules={() => useGame.getState().setRulesOpen(true)}
          onMenu={() => useGame.getState().abandon()}
          onToggle={(id) => useGame.getState().toggleCard(id)}
          onConfirmPass={() => {
            if (!settings.muted) sfxPass();
            useGame.getState().confirmPass();
          }}
          onPlay={(id) => {
            const ok = useGame.getState().play(id);
            if (!settings.muted) {
              if (ok) sfxCard();
              else sfxIllegal();
            }
          }}
          onScores={() => useGame.getState().setScoreOpen(true)}
          onLocale={(locale) => useGame.getState().setSettings({ locale })}
        />
      )}

      <RulesPanel
        open={rulesOpen}
        locale={settings.locale}
        onClose={() => useGame.getState().setRulesOpen(false)}
      />

      {state ? (
        <ScoreSheet
          open={scoreOpen && (state.phase === "handEnd" || state.phase === "gameOver")}
          state={state}
          names={names}
          locale={settings.locale}
          onClose={() => useGame.getState().setScoreOpen(false)}
          onNext={() => useGame.getState().nextHand()}
          onMenu={() => useGame.getState().abandon()}
          onAgain={() => useGame.getState().newGame()}
        />
      ) : null}
    </>
  );
}
