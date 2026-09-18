import { BookOpen, Play, Volume2, VolumeX } from "lucide-react";
import { Button } from "@/components/ui/button";
import { sfxCard, unlockAudio } from "@/game/audio";
import { t } from "@/game/i18n";
import type { Difficulty, VariantId } from "@/game/types";
import { VARIANTS } from "@/game/variants";
import { cn } from "@/lib/utils";
import type { CardBackTint, Settings, Stats } from "@/game/storage";
import { PlayingCard } from "./PlayingCard";
import { LanguageToggle } from "./LanguageToggle";
import { SuitMark } from "./suits";

type Props = {
  settings: Settings;
  stats: Stats;
  canContinue: boolean;
  onSettings: (patch: Partial<Settings>) => void;
  onStart: (variant?: VariantId) => void;
  onContinue: () => void;
  onRules: () => void;
};

export function MenuScreen({
  settings,
  stats,
  canContinue,
  onSettings,
  onStart,
  onContinue,
  onRules,
}: Props) {
  const copy = t(settings.locale);
  const variantCopy = copy.variant[settings.variant];
  const queenLetter = copy.face.queen;

  return (
    <main className="relative min-h-dvh overflow-hidden bg-felt-deep text-cream">
      <div className="felt-grain pointer-events-none absolute inset-0 opacity-60" />
      <div className="relative mx-auto flex min-h-dvh w-full max-w-lg flex-col justify-center px-5 py-8 sm:py-12">
        <header className="mb-8 sm:mb-10">
          <div className="flex items-start justify-between gap-3">
            <p className="text-xs font-medium uppercase tracking-[0.22em] text-muted">{copy.cardGame}</p>
            <LanguageToggle
              locale={settings.locale}
              onChange={(locale) => onSettings({ locale })}
              compact
            />
          </div>
          <h1 className="mt-2 font-display text-5xl leading-[0.95] tracking-tight sm:text-6xl">
            {copy.appName}
          </h1>
          <p className="mt-3 max-w-sm text-sm leading-relaxed text-muted">{copy.tagline}</p>
        </header>

        <div className="mb-6 flex justify-center">
          <HeroCard letter={queenLetter} locale={settings.locale} />
        </div>

        <div className="grid grid-cols-2 gap-2">
          {(Object.keys(VARIANTS) as VariantId[]).map((id) => {
            const v = VARIANTS[id];
            const labels = copy.variant[id];
            const active = settings.variant === id;
            return (
              <button
                key={id}
                type="button"
                onClick={() => onSettings({ variant: id })}
                className={cn(
                  "rounded-[var(--radius-lg)] border px-3 py-3 text-left transition-[background-color,border-color] duration-[var(--motion-quick)]",
                  active
                    ? "border-cream bg-cream text-ink"
                    : "border-line bg-felt-mid/60 text-cream hover:bg-felt-mid",
                )}
              >
                <div className="font-display text-lg leading-tight">{labels.name}</div>
                <div className={cn("mt-1 text-xs", active ? "text-ink-soft" : "text-muted")}>
                  {labels.short} · {copy.toLimit(v.gameLimit)}
                </div>
              </button>
            );
          })}
        </div>
        <p className="mt-2 text-xs leading-relaxed text-muted">{variantCopy.blurb}</p>

        <label className="mt-5 block text-xs font-medium text-muted">
          {copy.yourName}
          <input
            value={settings.playerName}
            placeholder={copy.you}
            maxLength={16}
            onChange={(e) => onSettings({ playerName: e.target.value })}
            className="mt-1 h-11 w-full rounded-[var(--radius-md)] border border-line bg-felt px-3 text-sm text-cream placeholder:text-muted focus:border-cream focus:outline-none"
          />
        </label>

        <fieldset className="mt-4">
          <legend className="text-xs font-medium text-muted">{copy.opponents}</legend>
          <div className="mt-1 grid grid-cols-3 gap-1.5">
            {(["easy", "normal", "hard"] as Difficulty[]).map((d) => {
              const label = copy[d];
              const active = settings.difficulty === d;
              return (
                <button
                  key={d}
                  type="button"
                  onClick={() => onSettings({ difficulty: d })}
                  className={cn(
                    "h-10 rounded-[var(--radius-sm)] border text-sm",
                    active ? "border-cream bg-cream text-ink" : "border-line text-cream hover:bg-felt-mid",
                  )}
                >
                  {label}
                </button>
              );
            })}
          </div>
        </fieldset>

        <fieldset className="mt-4">
          <legend className="text-xs font-medium text-muted">{copy.cardBack}</legend>
          <div className="mt-1 grid grid-cols-2 gap-1.5">
            {(["red", "blue"] as CardBackTint[]).map((tint) => {
              const active = settings.cardBack === tint;
              return (
                <button
                  key={tint}
                  type="button"
                  onClick={() => onSettings({ cardBack: tint })}
                  aria-pressed={active}
                  className={cn(
                    "flex h-10 items-center justify-center gap-2 rounded-[var(--radius-sm)] border text-sm",
                    active ? "border-cream bg-cream text-ink" : "border-line text-cream hover:bg-felt-mid",
                  )}
                >
                  <span
                    className={cn(
                      "size-3.5 rounded-full",
                      tint === "red" ? "bg-back-red" : "bg-back-blue",
                    )}
                    aria-hidden="true"
                  />
                  {copy.cardBackTint[tint]}
                </button>
              );
            })}
          </div>
        </fieldset>

        <div className="mt-8 flex flex-col gap-2">
          <Button size="lg" onClick={() => onStart()} className="w-full" data-testid="new-game">
            <Play className="size-4" />
            {copy.newGame}
          </Button>
          {canContinue ? (
            <Button variant="secondary" size="lg" onClick={onContinue} className="w-full">
              {copy.continue}
            </Button>
          ) : null}
          <div className="grid grid-cols-2 gap-2">
            <Button variant="ghost" onClick={onRules}>
              <BookOpen className="size-4" />
              {copy.rules}
            </Button>
            <Button
              variant="ghost"
              onClick={() => {
                const next = !settings.muted;
                onSettings({ muted: next });
                if (!next) {
                  unlockAudio();
                  sfxCard();
                }
              }}
              aria-pressed={settings.muted}
            >
              {settings.muted ? <VolumeX className="size-4" /> : <Volume2 className="size-4" />}
              {settings.muted ? copy.soundOff : copy.soundOn}
            </Button>
          </div>
        </div>

        {stats.gamesPlayed > 0 ? (
          <p className="mt-auto pt-8 text-xs text-muted">
            {copy.gamesWon(stats.gamesWon, stats.gamesPlayed)}
          </p>
        ) : (
          <div className="mt-auto pt-8" />
        )}
      </div>
    </main>
  );
}

function HeroCard({ letter, locale }: { letter: string; locale: Settings["locale"] }) {
  return (
    <div className="relative h-40 w-[7.2rem] rotate-[-8deg] rounded-[0.9rem] bg-card shadow-[0_18px_40px_rgba(0,0,0,0.45)]">
      <div className="absolute left-3 top-3 flex flex-col items-center text-ink">
        <span className="font-display text-2xl leading-none">{letter}</span>
        <SuitMark suit="spades" className="w-5 text-ink" />
      </div>
      <div className="absolute inset-0 grid place-items-center">
        <SuitMark suit="spades" className="w-16 text-ink" />
      </div>
      <div className="absolute bottom-3 right-3 flex rotate-180 flex-col items-center text-ink">
        <span className="font-display text-2xl leading-none">{letter}</span>
        <SuitMark suit="spades" className="w-5 text-ink" />
      </div>
      <div className="absolute -right-10 top-8 rotate-[14deg] shadow-lg">
        <PlayingCard faceDown size="lg" locale={locale} />
      </div>
    </div>
  );
}
