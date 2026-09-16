import { LOCALES, type Locale } from "@/game/i18n";
import { cn } from "@/lib/utils";

type Props = {
  locale: Locale;
  onChange: (locale: Locale) => void;
  compact?: boolean;
};

export function LanguageToggle({ locale, onChange, compact }: Props) {
  return (
    <div
      role="group"
      aria-label={locale === "nb" ? "Språk" : "Language"}
      className={cn("grid grid-cols-2 gap-1", compact ? "w-[8.75rem]" : "w-full")}
    >
      {LOCALES.map((item) => {
        const active = locale === item.id;
        return (
          <button
            key={item.id}
            type="button"
            onClick={() => onChange(item.id)}
            aria-pressed={active}
            data-testid={`locale-${item.id}`}
            className={cn(
              "h-10 rounded-[var(--radius-sm)] border text-sm font-medium",
              active
                ? "border-cream bg-cream text-ink"
                : "border-line text-cream hover:bg-felt-mid",
            )}
          >
            {item.label}
          </button>
        );
      })}
    </div>
  );
}
