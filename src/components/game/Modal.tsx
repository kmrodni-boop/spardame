import type { ReactNode } from "react";
import { X } from "lucide-react";
import { cn } from "@/lib/utils";

type Props = {
  open: boolean;
  title: string;
  onClose: () => void;
  children: ReactNode;
  footer?: ReactNode;
  wide?: boolean;
  closeLabel?: string;
};

export function Modal({ open, title, onClose, children, footer, wide, closeLabel = "Close" }: Props) {
  if (!open) return null;
  return (
    <div className="fixed inset-0 z-50 grid place-items-end sm:place-items-center p-0 sm:p-6">
      <button
        type="button"
        className="absolute inset-0 bg-ink/55"
        aria-label={closeLabel}
        onClick={onClose}
      />
      <div
        role="dialog"
        aria-modal="true"
        aria-labelledby="modal-title"
        className={cn(
          "relative z-10 w-full max-h-[92dvh] overflow-auto bg-cream text-ink shadow-[0_24px_60px_rgba(0,0,0,0.4)]",
          "rounded-t-[var(--radius-xl)] sm:rounded-[var(--radius-xl)]",
          wide ? "max-w-2xl" : "max-w-lg",
        )}
      >
        <div className="flex items-start justify-between gap-4 px-5 pt-5 pb-3 sm:px-7">
          <h2 id="modal-title" className="font-display text-2xl tracking-tight">
            {title}
          </h2>
          <button
            type="button"
            onClick={onClose}
            className="grid size-11 place-items-center rounded-[var(--radius-sm)] text-ink-soft hover:bg-cream-dim"
            aria-label={closeLabel}
          >
            <X className="size-5" />
          </button>
        </div>
        <div className="px-5 pb-5 sm:px-7 sm:pb-7">{children}</div>
        {footer ? (
          <div className="flex flex-col-reverse gap-2 border-t border-card-edge px-5 py-4 sm:flex-row sm:justify-end sm:px-7">
            {footer}
          </div>
        ) : null}
      </div>
    </div>
  );
}
