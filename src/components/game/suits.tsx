import type { Suit } from "@/game/types";
import { cn } from "@/lib/utils";

type Props = {
  suit: Suit;
  className?: string;
};

export function SuitMark({ suit, className }: Props) {
  const isRed = suit === "hearts" || suit === "diamonds";
  return (
    <svg
      viewBox="0 0 24 24"
      className={cn(isRed ? "text-heart" : "text-ink", className)}
      aria-hidden="true"
    >
      {suit === "spades" && (
        <path
          fill="currentColor"
          d="M12 2C9.2 6.1 4 10.4 4 14.2 4 17.3 6.3 19.4 9 19.4c.8 0 1.5-.2 2.1-.5L10.4 22h3.2l-.7-3.1c.6.3 1.3.5 2.1.5 2.7 0 5-2.1 5-5.2C20 10.4 14.8 6.1 12 2Z"
        />
      )}
      {suit === "hearts" && (
        <path
          fill="currentColor"
          d="M12 21s-7.5-4.8-9.4-9.3C1.2 8.4 2.9 5 6.2 5c1.9 0 3.4 1.1 4.3 2.6C11.4 6.1 12.9 5 14.8 5c3.3 0 5 3.4 3.6 6.7C19.5 16.2 12 21 12 21Z"
        />
      )}
      {suit === "diamonds" && (
        <path fill="currentColor" d="M12 2.2 20.4 12 12 21.8 3.6 12 12 2.2Z" />
      )}
      {suit === "clubs" && (
        <path
          fill="currentColor"
          d="M12 4.2a3.6 3.6 0 0 1 1.8 6.7 3.7 3.7 0 1 1-2.9 6.4L10.4 22h3.2l-.6-4.2A3.7 3.7 0 1 1 10.2 11 3.6 3.6 0 0 1 12 4.2Z"
        />
      )}
    </svg>
  );
}
