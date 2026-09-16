import { t, type Locale } from "@/game/i18n";
import { Modal } from "./Modal";

type Props = { open: boolean; onClose: () => void; locale: Locale };

export function RulesPanel({ open, onClose, locale }: Props) {
  const copy = t(locale);
  return (
    <Modal open={open} onClose={onClose} title={copy.rulesTitle} closeLabel={copy.close} wide>
      <div className="space-y-6 text-sm leading-relaxed text-ink-soft">
        <section className="space-y-2">
          <h3 className="font-display text-lg text-ink">{copy.rulesSpardameTitle}</h3>
          <p>{copy.rulesSpardameBody}</p>
          <ul className="list-disc space-y-1 pl-5">
            {copy.rulesSpardamePoints.map((item) => (
              <li key={item}>{item}</li>
            ))}
          </ul>
          <p>{copy.rulesSpardameExtra}</p>
        </section>

        <section className="space-y-2">
          <h3 className="font-display text-lg text-ink">{copy.rulesHeartsTitle}</h3>
          <p>{copy.rulesHeartsBody}</p>
        </section>

        <section className="space-y-2">
          <h3 className="font-display text-lg text-ink">{copy.rulesPlayTitle}</h3>
          <ul className="list-disc space-y-1 pl-5">
            {copy.rulesPlay.map((item) => (
              <li key={item}>{item}</li>
            ))}
          </ul>
        </section>
      </div>
    </Modal>
  );
}
