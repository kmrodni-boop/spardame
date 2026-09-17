import { cardLabel, rankCode } from "@/game/cards";
import { t, type Locale } from "@/game/i18n";
import type { Card } from "@/game/types";
import { cn } from "@/lib/utils";
import { SuitMark } from "./suits";

type Size = "xs" | "sm" | "md" | "lg" | "xl";

const SIZE: Record<Size, string> = {
  xs: "w-[2.4rem] sm:w-11",
  sm: "w-14 sm:w-16",
  md: "w-[4.6rem] sm:w-[5.25rem]",
  lg: "w-[5.15rem] sm:w-[5.75rem]",
  xl: "w-24 sm:w-[7.25rem] lg:w-32",
};

const RADIUS: Record<Size, string> = {
  xs: "rounded-[0.4rem]",
  sm: "rounded-[0.48rem]",
  md: "rounded-[0.55rem]",
  lg: "rounded-[0.6rem]",
  xl: "rounded-[0.72rem]",
};

const INNER_RADIUS: Record<Size, string> = {
  xs: "rounded-[0.28rem]",
  sm: "rounded-[0.34rem]",
  md: "rounded-[0.4rem]",
  lg: "rounded-[0.42rem]",
  xl: "rounded-[0.52rem]",
};

const PIP: Record<number, { x: number; y: number; flip?: boolean }[]> = {
  1: [{ x: 50, y: 50 }],
  2: [
    { x: 50, y: 22 },
    { x: 50, y: 78, flip: true },
  ],
  3: [
    { x: 50, y: 20 },
    { x: 50, y: 50 },
    { x: 50, y: 80, flip: true },
  ],
  4: [
    { x: 32, y: 22 },
    { x: 68, y: 22 },
    { x: 32, y: 78, flip: true },
    { x: 68, y: 78, flip: true },
  ],
  5: [
    { x: 32, y: 22 },
    { x: 68, y: 22 },
    { x: 50, y: 50 },
    { x: 32, y: 78, flip: true },
    { x: 68, y: 78, flip: true },
  ],
  6: [
    { x: 32, y: 22 },
    { x: 68, y: 22 },
    { x: 32, y: 50 },
    { x: 68, y: 50 },
    { x: 32, y: 78, flip: true },
    { x: 68, y: 78, flip: true },
  ],
  7: [
    { x: 32, y: 20 },
    { x: 68, y: 20 },
    { x: 50, y: 36 },
    { x: 32, y: 50 },
    { x: 68, y: 50 },
    { x: 32, y: 80, flip: true },
    { x: 68, y: 80, flip: true },
  ],
  8: [
    { x: 32, y: 18 },
    { x: 68, y: 18 },
    { x: 32, y: 40 },
    { x: 68, y: 40 },
    { x: 32, y: 60, flip: true },
    { x: 68, y: 60, flip: true },
    { x: 32, y: 82, flip: true },
    { x: 68, y: 82, flip: true },
  ],
  9: [
    { x: 32, y: 18 },
    { x: 68, y: 18 },
    { x: 32, y: 38 },
    { x: 68, y: 38 },
    { x: 50, y: 50 },
    { x: 32, y: 62, flip: true },
    { x: 68, y: 62, flip: true },
    { x: 32, y: 82, flip: true },
    { x: 68, y: 82, flip: true },
  ],
  10: [
    { x: 32, y: 16 },
    { x: 68, y: 16 },
    { x: 50, y: 28 },
    { x: 32, y: 38 },
    { x: 68, y: 38 },
    { x: 32, y: 62, flip: true },
    { x: 68, y: 62, flip: true },
    { x: 50, y: 72, flip: true },
    { x: 32, y: 84, flip: true },
    { x: 68, y: 84, flip: true },
  ],
};

function faceLetter(card: Card, locale: Locale): string {
  const face = t(locale).face;
  if (card.rank === 11) return face.jack;
  if (card.rank === 12) return face.queen;
  if (card.rank === 13) return face.king;
  if (card.rank === 14) return face.ace;
  return rankCode(card.rank);
}

type Props = {
  card?: Card;
  faceDown?: boolean;
  size?: Size;
  selected?: boolean;
  dimmed?: boolean;
  highlight?: boolean;
  onClick?: () => void;
  disabled?: boolean;
  locale?: Locale;
  className?: string;
};

export function PlayingCard({
  card,
  faceDown,
  size = "md",
  selected,
  dimmed,
  highlight,
  onClick,
  disabled,
  locale = "en",
  className,
}: Props) {
  const copy = t(locale);
  const clickable = Boolean(onClick) && !disabled;

  return (
    <button
      type="button"
      onClick={onClick}
      disabled={!clickable}
      data-card={card?.id ?? "back"}
      aria-label={card && !faceDown ? cardLabel(card, locale) : copy.cardBack}
      className={cn(
        "playing-card relative aspect-[5/7] select-none bg-transparent p-0 appearance-none disabled:opacity-100",
        SIZE[size],
        RADIUS[size],
        clickable && "cursor-pointer hover:-translate-y-2",
        selected && "-translate-y-3 sm:-translate-y-4 z-20",
        dimmed && "opacity-45",
        highlight && "card-received ring-2 ring-cream",
        !clickable && "cursor-default",
        className,
      )}
    >
      {faceDown || !card ? (
        <CardBack size={size} />
      ) : (
        <CardFace card={card} locale={locale} size={size} />
      )}
    </button>
  );
}

function CardBack({ size }: { size: Size }) {
  return (
    <div
      className={cn(
        "absolute inset-0 overflow-hidden border border-line bg-felt-deep",
        RADIUS[size],
      )}
    >
      <div
        className={cn("absolute inset-[5%] border border-cream/25", INNER_RADIUS[size])}
        style={{
          backgroundImage:
            "repeating-linear-gradient(135deg, color-mix(in oklab, var(--color-cream) 16%, transparent) 0 2px, transparent 2px 7px)",
        }}
      />
      <div className="absolute inset-0 grid place-items-center">
        <SuitMark suit="spades" className="w-[42%] text-cream/80" />
      </div>
    </div>
  );
}

function CardFace({ card, locale, size }: { card: Card; locale: Locale; size: Size }) {
  const letter = faceLetter(card, locale);
  const isRed = card.suit === "hearts" || card.suit === "diamonds";
  const pipSize =
    size === "xs" ? "w-2" : size === "sm" ? "w-2.5" : size === "xl" ? "w-4 sm:w-[1.15rem]" : "w-3.5";
  const cornerSize =
    size === "xs"
      ? "text-[0.55rem]"
      : size === "sm"
        ? "text-[0.7rem]"
        : size === "xl"
          ? "text-[0.92rem] sm:text-base"
          : "text-[0.82rem]";
  const faceType =
    size === "xs"
      ? "text-sm"
      : size === "sm"
        ? "text-lg"
        : size === "xl"
          ? "text-4xl sm:text-5xl"
          : "text-2xl";
  const faceSuit = size === "xs" ? "w-3" : size === "sm" ? "w-4" : size === "xl" ? "w-8 sm:w-10" : "w-6";
  const isFace = card.rank >= 11;
  const isAce = card.rank === 14;
  const pips = PIP[isAce ? 1 : card.rank] ?? [];

  return (
    <div
      className={cn(
        "playing-card-face absolute inset-0 overflow-hidden border text-ink",
        RADIUS[size],
        size === "xl" && "is-xl",
        isRed ? "border-heart/25" : "border-card-edge",
      )}
    >
      <Corner letter={letter} suit={card.suit} className={cn("left-[5%] top-[4%]", cornerSize)} />
      <Corner
        letter={letter}
        suit={card.suit}
        className={cn("right-[5%] bottom-[4%] rotate-180", cornerSize)}
      />
      {isFace ? (
        <div
          className={cn(
            "absolute inset-[16%_14%] grid place-items-center border border-card-edge/80 bg-[linear-gradient(180deg,#fff,color-mix(in_oklab,var(--color-cream)_55%,white))]",
            INNER_RADIUS[size],
          )}
        >
          <div className="flex flex-col items-center gap-0.5">
            <span
              className={cn("font-display leading-none", faceType, isRed ? "text-heart" : "text-ink")}
            >
              {letter}
            </span>
            <SuitMark suit={card.suit} className={faceSuit} />
          </div>
        </div>
      ) : (
        <div className="absolute inset-[16%_18%]">
          {pips.map((pip, i) => (
            <span
              key={i}
              className="absolute"
              style={{
                left: `${pip.x}%`,
                top: `${pip.y}%`,
                transform: `translate(-50%, -50%)${pip.flip ? " rotate(180deg)" : ""}`,
              }}
            >
              <SuitMark
                suit={card.suit}
                className={isAce && size !== "xs" ? (size === "xl" ? "w-10 sm:w-12" : "w-8 sm:w-9") : pipSize}
              />
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

function Corner({ letter, suit, className }: { letter: string; suit: Card["suit"]; className?: string }) {
  return (
    <div className={cn("absolute z-10 flex flex-col items-center leading-none", className)}>
      <span className="font-display font-semibold">{letter}</span>
      <SuitMark suit={suit} className="w-[0.85em]" />
    </div>
  );
}
