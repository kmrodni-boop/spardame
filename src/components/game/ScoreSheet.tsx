import { t, type Locale } from "@/game/i18n";
import type { GameState, PlayerId } from "@/game/types";
import { getVariant } from "@/game/variants";
import { Button } from "@/components/ui/button";
import { Modal } from "./Modal";

type Props = {
  open: boolean;
  state: GameState;
  names: string[];
  locale: Locale;
  onClose: () => void;
  onNext: () => void;
  onMenu: () => void;
  onAgain: () => void;
};

export function ScoreSheet({
  open,
  state,
  names,
  locale,
  onClose,
  onNext,
  onMenu,
  onAgain,
}: Props) {
  const copy = t(locale);
  const v = getVariant(state.variant);
  const over = state.phase === "gameOver";
  const hs = state.handScore;
  const moonName = hs?.moon != null ? names[hs.moon] : null;

  let title = copy.scoreHand;
  if (over) {
    if (state.tied.length > 1) title = copy.draw;
    else if (state.winner === 0) title = copy.youWon;
    else title = copy.won(names[state.winner ?? 0]!);
  }

  return (
    <Modal
      open={open}
      onClose={over ? onMenu : onClose}
      title={title}
      closeLabel={copy.close}
      footer={
        over ? (
          <>
            <Button variant="secondary" onClick={onMenu}>
              {copy.menu}
            </Button>
            <Button onClick={onAgain}>{copy.newGame}</Button>
          </>
        ) : (
          <>
            <Button variant="secondary" onClick={onMenu}>
              {copy.quit}
            </Button>
            <Button onClick={onNext}>{copy.nextHand}</Button>
          </>
        )
      }
    >
      {moonName ? (
        <p className="mb-4 rounded-[var(--radius-md)] bg-felt-deep px-3 py-2 text-sm text-cream">
          {copy.moon(moonName)}
        </p>
      ) : null}

      <div className="overflow-hidden rounded-[var(--radius-md)] border border-card-edge">
        <table className="w-full text-sm">
          <thead className="bg-felt-deep text-cream">
            <tr>
              <th className="px-3 py-2 text-left font-medium">{copy.player}</th>
              <th className="px-3 py-2 text-right font-medium">{copy.round}</th>
              <th className="px-3 py-2 text-right font-medium">{copy.total}</th>
            </tr>
          </thead>
          <tbody>
            {([0, 1, 2, 3] as PlayerId[]).map((p) => {
              const lowest = over && state.tied.includes(p);
              return (
                <tr
                  key={p}
                  className={lowest ? "bg-felt-deep/10 font-medium" : "odd:bg-cream-dim/40"}
                >
                  <td className="px-3 py-2.5">{names[p]}</td>
                  <td className="px-3 py-2.5 text-right tabular-nums">{fmt(hs?.applied[p] ?? 0)}</td>
                  <td className="px-3 py-2.5 text-right tabular-nums">
                    {state.scores[p]}
                    {state.scores[p]! >= v.gameLimit ? ` · ${copy.out}` : ""}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
      <p className="mt-3 text-xs text-ink-soft">{copy.scoreHint(v.gameLimit)}</p>
    </Modal>
  );
}

function fmt(n: number): string {
  if (n > 0) return `+${n}`;
  return String(n);
}
