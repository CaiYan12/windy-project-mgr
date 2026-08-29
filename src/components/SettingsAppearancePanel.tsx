import { useEffect, useRef, useState, type CSSProperties, type KeyboardEvent } from "react";
import type { AccentColor, ColorMode } from "../lib/api";
import { Icon } from "./Icon";
import {
  ACCENT_PRESETS,
  THEMES,
  isValidHexColor,
  resolveAccentColor,
} from "../lib/theme";
import {
  BODY_FONT_OPTIONS,
  MAX_FONT_SIZE,
  MAX_RADIUS,
  MIN_FONT_SIZE,
  MIN_RADIUS,
  STYLE_PRESETS,
  materializePreset,
  sanitizeFontFamily,
  type AppearanceSettings,
  type StylePreset,
} from "../lib/appearance";
import { nextRadioIndex } from "../lib/settingsUi";

interface SettingsAppearancePanelProps {
  colorMode: ColorMode;
  accentColor: AccentColor;
  appearance: AppearanceSettings;
  customAccentValue: string;
  windowsAccentColor: string | null;
  windowsAccentLoading: boolean;
  windowsAccentError: string | null;
  onColorModeChange: (colorMode: ColorMode) => void;
  onAccentChange: (accentColor: AccentColor) => void;
  onAppearanceChange: (appearance: AppearanceSettings) => void;
  onCustomAccentChange: (value: string) => void;
  onStylePresetSelect: (preset: StylePreset) => void;
  onRetryWindowsAccent: () => void;
}

/** 手写字体下拉：每个选项以自身字体栈渲染预览；自定义栈走下方输入框。 */
function FontFamilySelect({
  value,
  onChange,
}: {
  value: string;
  onChange: (stack: string) => void;
}) {
  const [open, setOpen] = useState(false);
  const triggerRef = useRef<HTMLButtonElement | null>(null);
  const optionRefs = useRef<Array<HTMLButtonElement | null>>([]);

  const selectedOption =
    BODY_FONT_OPTIONS.find((option) => option.stack === value) ?? null;
  const isCustom = selectedOption === null;

  useEffect(() => {
    if (!open) {
      return;
    }
    const focusTarget = isCustom ? 0 : BODY_FONT_OPTIONS.indexOf(selectedOption);
    optionRefs.current[focusTarget]?.focus();
  }, [open, isCustom, selectedOption]);

  function close(refocus: boolean) {
    setOpen(false);
    if (refocus) {
      triggerRef.current?.focus();
    }
  }

  function select(optionIndex: number) {
    onChange(BODY_FONT_OPTIONS[optionIndex].stack);
    close(true);
  }

  function onOptionKeyDown(event: KeyboardEvent<HTMLButtonElement>, index: number) {
    if (event.key === "Escape") {
      event.preventDefault();
      close(true);
      return;
    }
    const next = nextRadioIndex(index, event.key, BODY_FONT_OPTIONS.length);
    if (next !== null) {
      event.preventDefault();
      optionRefs.current[next]?.focus();
    }
  }

  return (
    <div className="settings-font-select">
      <button
        ref={triggerRef}
        type="button"
        className="settings-font-trigger"
        aria-haspopup="listbox"
        aria-expanded={open}
        aria-controls="settings-font-listbox"
        onClick={() => setOpen((v) => !v)}
        onKeyDown={(event) => {
          if (!open && (event.key === "ArrowDown" || event.key === "Enter" || event.key === " ")) {
            event.preventDefault();
            setOpen(true);
          }
        }}
      >
        <span
          className="settings-font-trigger-value"
          style={selectedOption?.stack ? { fontFamily: selectedOption.stack } : undefined}
        >
          {selectedOption?.label ?? (value === "" ? "System default" : `Custom: ${value}`)}
        </span>
        <Icon name="chevron-down" size={15} aria-hidden="true" />
      </button>
      {open && (
        <>
          <div className="menu-overlay" onClick={() => close(false)} />
          <div
            id="settings-font-listbox"
            className="settings-font-list"
            role="listbox"
            aria-label="Body font"
          >
            {BODY_FONT_OPTIONS.map((option, index) => (
              <button
                key={option.id}
                ref={(element) => {
                  optionRefs.current[index] = element;
                }}
                type="button"
                role="option"
                aria-selected={option.stack === value}
                className="settings-font-option"
                onClick={() => select(index)}
                onKeyDown={(event) => onOptionKeyDown(event, index)}
              >
                <span
                  className="settings-font-option-label"
                  style={option.stack ? { fontFamily: option.stack } : undefined}
                >
                  {option.label}
                </span>
                {option.stack === value && <Icon name="check-circle" size={15} />}
              </button>
            ))}
          </div>
        </>
      )}
    </div>
  );
}

export function SettingsAppearancePanel({
  colorMode,
  accentColor,
  appearance,
  customAccentValue,
  windowsAccentColor,
  windowsAccentLoading,
  windowsAccentError,
  onColorModeChange,
  onAccentChange,
  onAppearanceChange,
  onCustomAccentChange,
  onStylePresetSelect,
  onRetryWindowsAccent,
}: SettingsAppearancePanelProps) {
  const [customStackDraft, setCustomStackDraft] = useState(appearance.fontFamily);
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
  const presetRefs = useRef<Array<HTMLButtonElement | null>>([]);

  useEffect(() => {
    setCustomStackDraft(appearance.fontFamily);
  }, [appearance.fontFamily]);

  function patchAppearance(patch: Partial<AppearanceSettings>) {
    onAppearanceChange({ ...appearance, ...patch });
  }

  function changeCustomStack(raw: string) {
    setCustomStackDraft(raw);
    patchAppearance({ fontFamily: sanitizeFontFamily(raw) });
  }

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

  function onPresetKeyDown(event: KeyboardEvent<HTMLButtonElement>, index: number) {
    const next = nextRadioIndex(index, event.key, STYLE_PRESETS.length);
    if (next === null) {
      return;
    }
    event.preventDefault();
    onStylePresetSelect(STYLE_PRESETS[next]);
    presetRefs.current[next]?.focus();
  }

  return (
    <div className="settings-panel settings-appearance-panel">
      <section className="settings-section" aria-labelledby="settings-color-mode-title">
        <div className="settings-section-heading">
          <div>
            <h3 id="settings-color-mode-title" className="settings-section-title">
              <Icon name="monitor" size={18} />
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
              <span className="settings-mode-icon" aria-hidden="true">
                <Icon
                  name={theme.value === "system" ? "monitor" : theme.value === "light" ? "sun" : "moon"}
                  size={20}
                />
              </span>
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
              <Icon name="palette" size={18} />
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
                <span className="settings-accent-card-label">{option.label}</span>
                {selected && <Icon name="check-circle" size={15} />}
              </button>
            );
          })}
        </div>

        <div className="settings-custom-field">
          <label htmlFor="settings-custom-accent">
            <Icon name="palette" size={15} />
            <span>Custom #RRGGBB</span>
          </label>
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
            <p id="settings-custom-accent-error" className="settings-error settings-inline-message">
              <Icon name="alert-triangle" size={15} />
              <span>{customError}</span>
            </p>
          )}
        </div>

        {windowsAccentLoading && <p className="settings-hint">Reading the Windows accent color…</p>}
        {windowsAccentError && (
          <div className="settings-inline-alert" role="alert">
            <div className="inline-alert-copy">
              <Icon name="alert-triangle" size={17} />
              <span>{windowsAccentError} Windy teal is being used as a fallback.</span>
            </div>
            <button type="button" className="link-btn" onClick={onRetryWindowsAccent}>
              <Icon name="refresh" size={14} />
              Try again
            </button>
          </div>
        )}
      </section>

      <section className="settings-section" aria-labelledby="settings-style-preset-title">
        <div className="settings-section-heading">
          <div>
            <h3 id="settings-style-preset-title" className="settings-section-title">
              <Icon name="layers" size={18} />
              Style presets
            </h3>
            <p className="settings-section-description">
              Each pack is a complete token set with a recommended accent and neutral palette.
            </p>
          </div>
        </div>
        <div className="settings-preset-grid" role="radiogroup" aria-label="Style presets">
          {STYLE_PRESETS.map((preset, index) => {
            const selected = appearance.stylePreset === preset.id;
            return (
              <button
                key={preset.id}
                ref={(element) => {
                  presetRefs.current[index] = element;
                }}
                type="button"
                className={`settings-preset-card${selected ? " is-selected" : ""}`}
                role="radio"
                aria-checked={selected}
                tabIndex={selected ? 0 : -1}
                onClick={() => onStylePresetSelect(preset)}
                onKeyDown={(event) => onPresetKeyDown(event, index)}
              >
                <span
                  className="settings-preset-swatch"
                  aria-hidden="true"
                  style={{
                    "--preset-bg": preset.neutrals.light.bg,
                    "--preset-surface": preset.neutrals.light.surface,
                    "--preset-sunken": preset.neutrals.light.sunken,
                    "--preset-line": preset.neutrals.light.line,
                  } as CSSProperties}
                />
                <span className="settings-preset-card-copy">
                  <span className="settings-preset-card-label">{preset.label}</span>
                  <span className="settings-preset-card-desc">{preset.description}</span>
                </span>
                {selected && <Icon name="check-circle" size={15} />}
              </button>
            );
          })}
        </div>
      </section>

      <section className="settings-section" aria-labelledby="settings-workshop-title">
        <div className="settings-section-heading">
          <div>
            <h3 id="settings-workshop-title" className="settings-section-title">
              <Icon name="settings" size={18} />
              Theme workshop
            </h3>
            <p className="settings-section-description">
              Fine-tune the current look with live preview. Save it as a custom theme or reset to
              the Windy default.
            </p>
          </div>
        </div>

        <div className="settings-slider-grid">
          <div className="settings-slider-row">
            <div className="settings-slider-head">
              <label className="settings-field-label" htmlFor="settings-radius-slider">
                Corner radius
              </label>
              <output className="settings-slider-value" htmlFor="settings-radius-slider">
                {appearance.radius}px
              </output>
            </div>
            <input
              id="settings-radius-slider"
              className="settings-slider"
              type="range"
              min={MIN_RADIUS}
              max={MAX_RADIUS}
              step={1}
              value={appearance.radius}
              aria-valuetext={`${appearance.radius} pixels`}
              onChange={(event) => patchAppearance({ radius: Number(event.target.value) })}
            />
          </div>
          <div className="settings-slider-row">
            <div className="settings-slider-head">
              <label className="settings-field-label" htmlFor="settings-font-size-slider">
                Base font size
              </label>
              <output className="settings-slider-value" htmlFor="settings-font-size-slider">
                {appearance.fontSize}px
              </output>
            </div>
            <input
              id="settings-font-size-slider"
              className="settings-slider"
              type="range"
              min={MIN_FONT_SIZE}
              max={MAX_FONT_SIZE}
              step={1}
              value={appearance.fontSize}
              aria-valuetext={`${appearance.fontSize} pixels`}
              onChange={(event) => patchAppearance({ fontSize: Number(event.target.value) })}
            />
          </div>
        </div>

        <div className="settings-workshop-field">
          <span className="settings-field-label" id="settings-density-label">
            Density
          </span>
          <div
            className="settings-segmented"
            role="radiogroup"
            aria-labelledby="settings-density-label"
          >
            {(["comfortable", "compact"] as const).map((density) => (
              <button
                key={density}
                type="button"
                role="radio"
                aria-checked={appearance.density === density}
                tabIndex={appearance.density === density ? 0 : -1}
                onClick={() => patchAppearance({ density })}
              >
                {density === "comfortable" ? "Comfortable" : "Compact"}
              </button>
            ))}
          </div>
        </div>

        <div className="settings-workshop-field">
          <span className="settings-field-label" id="settings-font-label">
            Body font
          </span>
          <FontFamilySelect
            value={appearance.fontFamily}
            onChange={(stack) => patchAppearance({ fontFamily: stack })}
          />
          <div className="settings-font-custom">
            <label className="settings-field-label" htmlFor="settings-font-custom">
              Custom font stack
            </label>
            <input
              id="settings-font-custom"
              type="text"
              spellCheck={false}
              value={customStackDraft}
              placeholder={'"Cascadia Code", Consolas, monospace'}
              aria-describedby="settings-font-custom-hint"
              onChange={(event) => changeCustomStack(event.target.value)}
            />
            <p id="settings-font-custom-hint" className="settings-hint">
              Leave empty to use the system default stack. Falls back automatically when fonts are
              missing.
            </p>
          </div>
        </div>

        <div className="settings-workshop-actions">
          <button
            type="button"
            className="btn small"
            disabled={appearance.stylePreset === null}
            onClick={() => patchAppearance({ stylePreset: null })}
            title={
              appearance.stylePreset === null
                ? "The current look is already a custom theme"
                : undefined
            }
          >
            <Icon name="check-circle" size={15} />
            <span>Save as custom theme</span>
          </button>
          <button
            type="button"
            className="btn small"
            onClick={() => onAppearanceChange(materializePreset(STYLE_PRESETS[0]))}
          >
            <Icon name="rotate-ccw" size={15} />
            <span>Reset appearance</span>
          </button>
          <span className="settings-workshop-hint">
            {appearance.stylePreset === null
              ? "Custom theme"
              : `Editing the ${STYLE_PRESETS.find((preset) => preset.id === appearance.stylePreset)?.label ?? ""} preset`}
          </span>
        </div>
      </section>
    </div>
  );
}
