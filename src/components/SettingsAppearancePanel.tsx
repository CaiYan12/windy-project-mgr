import { useRef, type CSSProperties, type KeyboardEvent } from "react";
import type { AccentColor, ColorMode } from "../lib/api";
import {
  ACCENT_PRESETS,
  THEMES,
  isValidHexColor,
  resolveAccentColor,
} from "../lib/theme";
import { nextRadioIndex } from "../lib/settingsUi";

interface SettingsAppearancePanelProps {
  colorMode: ColorMode;
  accentColor: AccentColor;
  customAccentValue: string;
  windowsAccentColor: string | null;
  windowsAccentLoading: boolean;
  windowsAccentError: string | null;
  onColorModeChange: (colorMode: ColorMode) => void;
  onAccentChange: (accentColor: AccentColor) => void;
  onCustomAccentChange: (value: string) => void;
  onRetryWindowsAccent: () => void;
}

export function SettingsAppearancePanel({
  colorMode,
  accentColor,
  customAccentValue,
  windowsAccentColor,
  windowsAccentLoading,
  windowsAccentError,
  onColorModeChange,
  onAccentChange,
  onCustomAccentChange,
  onRetryWindowsAccent,
}: SettingsAppearancePanelProps) {
  const windowsColor = resolveAccentColor({ kind: "windows" }, windowsAccentColor);
  const customColor = isValidHexColor(customAccentValue) ? customAccentValue : "#0E7D8C";
  const customError =
    accentColor.kind === "custom" && !isValidHexColor(customAccentValue)
      ? "Use a six-digit color in the format #RRGGBB."
      : null;
  const accentOptions = [
    ...ACCENT_PRESETS.map((preset) => ({
      id: preset.id,
      label: preset.label,
      selection: { kind: "preset", value: preset.id } as AccentColor,
      previewColor: preset.value,
      windows: false,
    })),
    {
      id: "windows",
      label: "Windows current",
      selection: { kind: "windows" } as AccentColor,
      previewColor: windowsColor,
      windows: true,
    },
    {
      id: "custom",
      label: "Custom",
      selection: { kind: "custom", value: customAccentValue } as AccentColor,
      previewColor: customColor,
      windows: false,
    },
  ];
  const accentRefs = useRef<Array<HTMLButtonElement | null>>([]);

  function isAccentSelected(selection: AccentColor): boolean {
    if (selection.kind === "windows") {
      return accentColor.kind === "windows";
    }
    return accentColor.kind === selection.kind && selection.value === accentColor.value;
  }

  function onAccentKeyDown(event: KeyboardEvent<HTMLButtonElement>, index: number) {
    const next = nextRadioIndex(index, event.key, accentOptions.length);
    if (next === null) {
      return;
    }
    event.preventDefault();
    onAccentChange(accentOptions[next].selection);
    accentRefs.current[next]?.focus();
  }

  return (
    <div className="settings-panel settings-appearance-panel">
      <section className="settings-section" aria-labelledby="settings-color-mode-title">
        <div className="settings-section-heading">
          <div>
            <h3 id="settings-color-mode-title" className="settings-section-title">
              Color mode
            </h3>
            <p className="settings-section-description">
              Choose how Windy follows the light around your workspace.
            </p>
          </div>
        </div>
        <div className="settings-mode-grid" role="radiogroup" aria-label="Color mode">
          {THEMES.map((theme) => (
            <label
              key={theme.value}
              className={`settings-mode-card${colorMode === theme.value ? " is-selected" : ""}`}
            >
              <input
                type="radio"
                name="settings-color-mode"
                value={theme.value}
                checked={colorMode === theme.value}
                onChange={() => onColorModeChange(theme.value)}
              />
              <span className={`settings-mode-swatch ${theme.value}`} aria-hidden="true" />
              <span className="settings-mode-copy">
                <strong>{theme.label}</strong>
                <small>
                  {theme.value === "system"
                    ? "Use Windows preference"
                    : theme.value === "light"
                      ? "Keep paper bright"
                      : "Keep the canvas dark"}
                </small>
              </span>
            </label>
          ))}
        </div>
      </section>

      <section className="settings-section" aria-labelledby="settings-accent-title">
        <div className="settings-section-heading">
          <div>
            <h3 id="settings-accent-title" className="settings-section-title">
              Accent color
            </h3>
            <p className="settings-section-description">
              Preview an accent across the dashboard. Changes stay local until you save.
            </p>
          </div>
          <span
            className="settings-accent-preview"
            style={{ "--settings-preview-color": resolveAccentColor(accentColor, windowsAccentColor) } as CSSProperties}
            aria-label={`Current accent ${resolveAccentColor(accentColor, windowsAccentColor)}`}
          />
        </div>

        <div className="settings-accent-grid" role="radiogroup" aria-label="Accent color">
          {accentOptions.map((option, index) => {
            const selected = isAccentSelected(option.selection);
            return (
              <button
                key={option.id}
                ref={(element) => {
                  accentRefs.current[index] = element;
                }}
                type="button"
                className={`settings-accent-card${selected ? " is-selected" : ""}`}
                role="radio"
                aria-checked={selected}
                tabIndex={selected ? 0 : -1}
                onClick={() => onAccentChange(option.selection)}
                onKeyDown={(event) => onAccentKeyDown(event, index)}
              >
                <span
                  className={`settings-accent-swatch${option.windows ? " settings-accent-swatch-windows" : ""}`}
                  style={{ "--settings-preview-color": option.previewColor } as CSSProperties}
                  aria-hidden="true"
                />
                <span>{option.label}</span>
              </button>
            );
          })}
        </div>

        <div className="settings-custom-field">
          <label htmlFor="settings-custom-accent">Custom #RRGGBB</label>
          <input
            id="settings-custom-accent"
            type="text"
            inputMode="text"
            spellCheck={false}
            value={customAccentValue}
            aria-invalid={customError !== null}
            aria-describedby={customError ? "settings-custom-accent-error" : undefined}
            placeholder="#0E7D8C"
            onFocus={() => {
              if (accentColor.kind !== "custom") {
                onAccentChange({ kind: "custom", value: customAccentValue });
              }
            }}
            onChange={(event) => onCustomAccentChange(event.target.value)}
          />
          {customError && (
            <p id="settings-custom-accent-error" className="settings-error">
              {customError}
            </p>
          )}
        </div>

        {windowsAccentLoading && <p className="settings-hint">Reading the Windows accent color…</p>}
        {windowsAccentError && (
          <div className="settings-inline-alert" role="alert">
            <span>{windowsAccentError} Windy teal is being used as a fallback.</span>
            <button type="button" className="link-btn" onClick={onRetryWindowsAccent}>
              Try again
            </button>
          </div>
        )}
      </section>
    </div>
  );
}
