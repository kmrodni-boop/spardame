import { useEffect, useState } from "react";
import { BookOpen, House, Volume2, VolumeX } from "lucide-react";
import {
  currentWinnerOfTrick,
  jackStillOut,
  legalMoves,
  pointCardsTaken,
  queenStillOut,
  suitLed,
} from "@/game/engine";
import { flashCopy, t, type Locale } from "@/game/i18n";
import { getVariant } from "@/game/variants";
import type { Flash, GameState, PlayerId, Suit } from "@/game/types";
import { PLAYERS } from "@/game/types";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { LanguageToggle } from "./LanguageToggle";
import { PlayingCard } from "./PlayingCard";
import { SuitMark } from "./suits";

type Props = {
  state: GameState;
  names: string[];
  selected: string[];
  flash: Flash | null;
  muted: boolean;
  locale: Locale;
  onToggleMute: () => void;
  onRules: () => void;
  onMenu: () => void;
  onToggle: (id: string) => void;
  onConfirmPass: () => void;
  onPlay: (id: string) => void;
  onScores: () => void;
  onLocale: (locale: Locale) => void;
};

const FLY_FROM: Record<PlayerId, string> = {
  0: "trick-from-0",
  1: "trick-from-1",
  2: "trick-from-2",
  3: "trick-from-3",
};

const COLLECT_TO: Record<PlayerId, string> = {
  0: "trick-to-0",
  1: "trick-to-1",
  2: "trick-to-2",
  3: "trick-to-3",
};

const TRICK_POS: Record<PlayerId, string> = {
  0: "bottom-[14%] left-1/2 -translate-x-1/2 rotate-[-8deg]",
  1: "left-[12%] top-[48%] -translate-y-1/2 rotate-[-16deg]",
  2: "top-[10%] left-1/2 -translate-x-1/2 rotate-[5deg]",
  3: "right-[12%] top-[48%] -translate-y-1/2 rotate-[14deg]",
};

const TABLE_SHAPE = "rounded-[42%_42%_38%_38%/48%_48%_40%_40%]";

export function GameTable(props: Props) {
  const { state, names, locale } = props;
  const legal = state.phase === "playing" && state.turn === 0 ? legalMoves(state, 0) : [];
  const legalIds = new Set(legal.map((c) => c.id));
  const lead = suitLed(state);
  const thinking = state.phase === "playing" && state.turn !== 0;
  const [collecting, setCollecting] = useState(false);
  const winner = state.phase === "trickEnd" ? currentWinnerOfTrick(state.trick) : null;

  useEffect(() => {
    if (state.phase !== "trickEnd") {
      setCollecting(false);
      return;
    }
    const delay = window.matchMedia("(prefers-reduced-motion: reduce)").matches ? 40 : 550;
    const timer = window.setTimeout(() => setCollecting(true), delay);
    return () => window.clearTimeout(timer);
  }, [state.phase, state.trickNumber, state.handNumber]);

  return (
    <div className="relative flex min-h-dvh flex-col overflow-x-hidden bg-felt-deep text-cream">
      <div className="felt-grain pointer-events-none absolute inset-0 opacity-55" />
      <Header {...props} />

      <div className="relative mx-auto grid w-full min-w-0 max-w-6xl flex-1 grid-cols-[3.15rem_minmax(0,1fr)_3.15rem] grid-rows-[auto_minmax(18rem,1fr)] gap-1 px-2 pb-[12rem] pt-1 sm:grid-cols-[7.5rem_minmax(0,1fr)_7.5rem] sm:gap-3 sm:px-5 sm:pb-[13rem] lg:grid-cols-[9rem_minmax(0,1fr)_9rem]">
        <div className="col-start-2 row-start-1 flex justify-center">
          <Seat
            player={2}
            name={names[2]!}
            state={state}
            thinking={thinking && state.turn === 2}
            locale={locale}
          />
        </div>
        <div className="col-start-1 row-start-2 flex items-center justify-center">
          <Seat
            player={1}
            name={names[1]!}
            state={state}
            thinking={thinking && state.turn === 1}
            locale={locale}
          />
        </div>
        <div className="col-start-3 row-start-2 flex items-center justify-center">
          <Seat
            player={3}
            name={names[3]!}
            state={state}
            thinking={thinking && state.turn === 3}
            locale={locale}
          />
        </div>

        <div className="relative z-10 col-start-2 row-start-2 mx-auto flex h-full w-full max-w-3xl items-stretch overflow-visible">
          <div className={cn("table-rail relative h-full min-h-[20rem] w-full overflow-visible sm:min-h-[26rem]", TABLE_SHAPE)}>
            <div className={cn("table-felt relative h-full w-full overflow-visible", TABLE_SHAPE)}>
              <div className="table-monogram" aria-hidden />
              <div className="absolute left-1/2 top-1/2 z-10 h-[16.5rem] w-[16.5rem] -translate-x-1/2 -translate-y-1/2 overflow-visible sm:h-[21rem] sm:w-[21rem] lg:h-[24rem] lg:w-[24rem]">
                {state.trick.length === 0 ? (
                  <div className="grid h-full place-items-center text-center">
                    <StatusGlyph state={state} lead={lead} locale={locale} />
                  </div>
                ) : (
                  PLAYERS.map((p) => {
                    const play = state.trick.find((item) => item.player === p);
                    if (!play) return null;
                    const winning =
                      currentWinnerOfTrick(state.trick) === p && state.trick.length > 1;
                    const order = state.trick.findIndex((item) => item.player === p);
                    return (
                      <div
                        key={play.card.id}
                        className={cn("pointer-events-none absolute", TRICK_POS[p])}
                        style={{ zIndex: order + 1 }}
                      >
                        <div
                          className={cn(
                            "trick-collect",
                            collecting && winner != null && ["collecting", COLLECT_TO[winner]],
                          )}
                          style={{ transitionDelay: collecting ? `${order * 45}ms` : "0ms" }}
                        >
                          <div className={cn("trick-fly", FLY_FROM[p])}>
                            <PlayingCard
                              card={play.card}
                              size="xl"
                              locale={locale}
                              className={winning ? "ring-2 ring-cream/75" : undefined}
                            />
                          </div>
                        </div>
                      </div>
                    );
                  })
                )}
              </div>
            </div>
          </div>
        </div>
      </div>

      <HandDock
        state={state}
        names={names}
        flash={props.flash}
        legalIds={legalIds}
        selected={props.selected}
        locale={locale}
        onToggle={props.onToggle}
        onPlay={props.onPlay}
        onConfirmPass={props.onConfirmPass}
      />
    </div>
  );
}

function Header({
  state,
  names,
  muted,
  locale,
  onToggleMute,
  onRules,
  onMenu,
  onScores,
  onLocale,
}: Props) {
  const copy = t(locale);
  const variantCopy = copy.variant[state.variant];
  return (
    <header className="relative z-20 flex items-center gap-2 px-3 py-2 sm:px-5">
      <button
        type="button"
        onClick={onMenu}
        className="grid size-11 place-items-center rounded-[var(--radius-sm)] hover:bg-felt-mid"
        aria-label={copy.toMenu}
      >
        <House className="size-5" />
      </button>
      <button
        type="button"
        onClick={onScores}
        className="flex min-w-0 flex-1 items-center gap-2 overflow-x-auto py-1"
      >
        {PLAYERS.map((p) => (
          <span
            key={p}
            className={cn(
              "flex items-center gap-1.5 rounded-full border px-2.5 py-1 text-xs tabular-nums",
              state.turn === p && state.phase === "playing"
                ? "border-cream bg-cream text-ink"
                : "border-line bg-felt-deep/50 text-cream",
            )}
          >
            <span className="max-w-[4.5rem] truncate">{names[p]}</span>
            <span className="font-medium">{state.scores[p]}</span>
          </span>
        ))}
      </button>
      <span className="hidden text-[11px] uppercase tracking-wider text-muted lg:inline">
        {variantCopy.short} · {state.handNumber + 1}
      </span>
      <div className="hidden sm:block">
        <LanguageToggle locale={locale} onChange={onLocale} compact />
      </div>
      <button
        type="button"
        onClick={() => onLocale(locale === "en" ? "nb" : "en")}
        className="grid h-11 min-w-11 place-items-center rounded-[var(--radius-sm)] px-2 text-xs font-medium uppercase tracking-wider hover:bg-felt-mid sm:hidden"
        aria-label={copy.language}
        data-testid="locale-cycle"
      >
        {locale === "en" ? "EN" : "NO"}
      </button>
      <button
        type="button"
        onClick={onToggleMute}
        className="grid size-11 place-items-center rounded-[var(--radius-sm)] hover:bg-felt-mid"
        aria-label={muted ? copy.soundOn : copy.soundOff}
      >
        {muted ? <VolumeX className="size-5" /> : <Volume2 className="size-5" />}
      </button>
      <button
        type="button"
        onClick={onRules}
        className="grid size-11 place-items-center rounded-[var(--radius-sm)] hover:bg-felt-mid"
        aria-label={copy.rules}
      >
        <BookOpen className="size-5" />
      </button>
    </header>
  );
}

function Seat({
  player,
  name,
  state,
  thinking,
  locale,
}: {
  player: PlayerId;
  name: string;
  state: GameState;
  thinking: boolean;
  locale: Locale;
}) {
  const count = state.hands[player]!.length;
  const points = pointCardsTaken(state.taken[player]!, state.variant);
  return (
    <div className="flex flex-col items-center gap-1.5">
      <div
        className={cn(
          "rounded-full border px-3 py-1 text-xs",
          thinking ? "border-cream bg-cream text-ink" : "border-line bg-felt-deep/70",
        )}
      >
        {name}
        {thinking ? " …" : ""}
      </div>
      <div className={cn("seat-pad rounded-[var(--radius-md)]")}>
        <div className="flex max-w-full items-end overflow-hidden">
          {Array.from({ length: Math.min(count, 6) }).map((_, i) => (
            <div key={i} className="-ml-5 first:ml-0 sm:-ml-5" style={{ zIndex: i }}>
              <PlayingCard faceDown size="xs" locale={locale} />
            </div>
          ))}
        </div>
      </div>
      {points.length > 0 ? (
        <div className="flex max-w-[7rem] flex-wrap justify-center gap-0.5">
          {points.slice(0, 6).map((c) => (
            <SuitMark key={c.id} suit={c.suit} className="w-3.5" />
          ))}
        </div>
      ) : null}
    </div>
  );
}

function HandDock({
  state,
  names,
  flash,
  legalIds,
  selected,
  locale,
  onToggle,
  onPlay,
  onConfirmPass,
}: {
  state: GameState;
  names: string[];
  flash: Flash | null;
  legalIds: Set<string>;
  selected: string[];
  locale: Locale;
  onToggle: (id: string) => void;
  onPlay: (id: string) => void;
  onConfirmPass: () => void;
}) {
  const copy = t(locale);
  const v = getVariant(state.variant);
  const passing = state.phase === "passing";
  const myTurn = state.phase === "playing" && state.turn === 0;
  const hand = state.hands[0]!;
  const received = new Set(state.receivedPass.map((c) => c.id));
  const status = flash ? flashCopy(locale, flash) : statusText(state, names, locale);

  return (
    <div className="hand-dock fixed inset-x-0 bottom-0 z-30 pb-[max(0.6rem,env(safe-area-inset-bottom))] pt-2">
      <p className="mx-auto max-w-xl px-4 pb-2 text-center text-sm text-cream">{status}</p>
      {passing ? (
        <div className="mb-3 flex justify-center px-4">
          <Button
            onClick={onConfirmPass}
            disabled={selected.length !== v.passCount}
            data-testid="confirm-pass"
          >
            {copy.sendCards(selected.length, v.passCount, copy.pass[state.passDir])}
          </Button>
        </div>
      ) : null}
      <div className="mx-auto flex w-full min-w-0 max-w-4xl items-end justify-center overflow-x-auto px-3 pb-1">
        {hand.map((card, i) => {
          const isSel = selected.includes(card.id);
          const dim = myTurn && !legalIds.has(card.id);
          return (
            <div
              key={card.id}
              className="hand-enter relative shrink-0"
              style={{
                marginLeft: i === 0 ? 0 : "-1.35rem",
                zIndex: isSel ? 40 : i,
                animationDelay: `${i * 32}ms`,
              }}
            >
              <PlayingCard
                card={card}
                size="lg"
                locale={locale}
                selected={isSel}
                dimmed={dim}
                highlight={
                  received.has(card.id) && state.trickNumber === 0 && state.trick.length === 0
                }
                onClick={() => {
                  if (passing) onToggle(card.id);
                  else if (myTurn) onPlay(card.id);
                }}
                disabled={!passing && !myTurn}
              />
            </div>
          );
        })}
      </div>
    </div>
  );
}

function StatusGlyph({
  state,
  lead,
  locale,
}: {
  state: GameState;
  lead: Suit | null;
  locale: Locale;
}) {
  const copy = t(locale);
  if (state.phase === "passing") {
    return <p className="text-xs text-muted">{copy.passing}</p>;
  }
  if (lead) {
    return (
      <div className="flex flex-col items-center gap-1 text-muted">
        <SuitMark suit={lead} className="w-10" />
        <span className="text-[11px] uppercase tracking-wider">{copy.suitCap[lead]}</span>
      </div>
    );
  }
  const bits = [];
  if (state.heartsBroken) bits.push(copy.heartsBroken);
  if (queenStillOut(state)) bits.push(copy.queenOut);
  if (state.variant === "spardame" && jackStillOut(state)) bits.push(copy.jackOut);
  return (
    <p className="max-w-[9rem] text-[11px] leading-snug text-muted">{bits.join(" · ") || copy.lead}</p>
  );
}

function statusText(state: GameState, names: string[], locale: Locale): string {
  const copy = t(locale);
  if (state.phase === "passing") {
    return copy.pickCards(getVariant(state.variant).passCount, copy.pass[state.passDir]);
  }
  if (state.phase === "trickEnd") {
    const w = currentWinnerOfTrick(state.trick);
    return w == null ? copy.trick : copy.takesTrick(names[w]!, w === 0);
  }
  if (state.phase === "handEnd") return copy.handOver;
  if (state.phase === "gameOver") return copy.gameOver;
  if (state.turn !== 0) return copy.thinking(names[state.turn]!);
  if (state.trick.length === 0) {
    if (state.trickNumber === 0) return copy.yourLeadTwo;
    return copy.yourLead;
  }
  const suit = state.trick[0]!.card.suit;
  const canFollow = state.hands[0]!.some((c) => c.suit === suit);
  if (canFollow) return copy.followSuit(copy.suit[suit]);
  return copy.voidIn(copy.suit[suit]);
}
