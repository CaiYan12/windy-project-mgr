import type {
  AccentColor,
  AccentPresetId,
  ColorMode,
  EditorProfile,
} from "./api";
import {
  APPEARANCE_VARIABLE_NAMES,
  deriveAppearanceVariables,
  type AppearanceSettings,
} from "./appearance";

export type { AccentColor, AccentPresetId, ColorMode, EditorProfile } from "./api";

/** Compatibility name retained for existing three-state theme consumers. */
export type Theme = ColorMode;

export const THEMES: ReadonlyArray<{ value: ColorMode; label: string }> = [
  { value: "system", label: "Follow system" },
  { value: "light", label: "Light" },
  { value: "dark", label: "Dark" },
];

export interface AccentPreset {
  readonly id: AccentPresetId;
  readonly label: string;
  readonly value: string;
}

/** Stable v2 preset metadata. The ids are persisted; the values are CSS colors. */
export const ACCENT_PRESETS: ReadonlyArray<AccentPreset> = [
  { id: "windy-teal", label: "Windy teal", value: "#0E7D8C" },
  { id: "ocean-blue", label: "Ocean blue", value: "#2563EB" },
  { id: "violet", label: "Violet", value: "#7C3AED" },
  { id: "amber", label: "Amber", value: "#D97706" },
  { id: "coral", label: "Coral", value: "#E05A47" },
  { id: "rose", label: "Rose", value: "#D94675" },
];

export const DEFAULT_ACCENT_COLOR: AccentColor = {
  kind: "preset",
  value: "windy-teal",
};

export const DEFAULT_EDITOR_ARGUMENTS: readonly ["{path}"] = ["{path}"];
export const EDITOR_PATH_PLACEHOLDER = "{path}" as const;

const DEFAULT_ACCENT_VALUE = "#0E7D8C";
const DARK_TEXT = "#111827";
const LIGHT_TEXT = "#FFFFFF";
const ACCENT_VARIABLE_NAMES = [
  "--accent",
  "--accent-hover",
  "--accent-soft",
  "--focus",
  "--on-accent",
  "--accent-ink",
] as const;

/** 主题预览需要捕获/还原的全部变量：accent 五项 + appearance 十项。 */
const THEME_VARIABLE_NAMES = [
  ...ACCENT_VARIABLE_NAMES,
  ...APPEARANCE_VARIABLE_NAMES,
] as const;

export type ThemeVariableName =
  | (typeof ACCENT_VARIABLE_NAMES)[number]
  | (typeof APPEARANCE_VARIABLE_NAMES)[number];

export type AccentCssVariables = Record<(typeof ACCENT_VARIABLE_NAMES)[number], string>;
export type EffectiveColorMode = Exclude<ColorMode, "system">;

export const PREFERS_DARK_QUERY = "(prefers-color-scheme: dark)";

export interface ThemeState {
  readonly colorMode: string | null;
  readonly variables: Readonly<Record<ThemeVariableName, string | null>>;
}

export function isValidColorMode(value: string): value is ColorMode {
  return value === "system" || value === "light" || value === "dark";
}

export function resolveEffectiveColorMode(
  colorMode: ColorMode,
  prefersDark = false,
): EffectiveColorMode {
  if (colorMode === "system") {
    return prefersDark ? "dark" : "light";
  }
  return colorMode;
}

/** Compatibility name retained for the old three-state theme callers. */
export const isValidTheme = isValidColorMode;

export function isValidHexColor(value: string): boolean {
  return /^#[0-9A-Fa-f]{6}$/.test(value);
}

export function parseAccentColor(value: unknown): AccentColor | null {
  if (!isRecord(value) || typeof value.kind !== "string") {
    return null;
  }

  if (value.kind === "windows") {
    return hasExactKeys(value, ["kind"]) ? { kind: "windows" } : null;
  }

  if (!hasExactKeys(value, ["kind", "value"]) || typeof value.value !== "string") {
    return null;
  }

  if (value.kind === "preset" && isAccentPresetId(value.value)) {
    return { kind: "preset", value: value.value };
  }
  if (value.kind === "custom" && isValidHexColor(value.value)) {
    return { kind: "custom", value: value.value };
  }
  return null;
}

/** Build a persisted accent selection while preserving the tagged union shape. */
export function selectAccentColor(
  kind: AccentColor["kind"],
  value?: string,
): AccentColor | null {
  if (kind === "windows") {
    return value === undefined ? { kind: "windows" } : null;
  }
  if (value === undefined) {
    return null;
  }
  return parseAccentColor({ kind, value });
}

export function resolveAccentColor(
  selection: AccentColor | string,
  windowsAccentColor?: string | null,
): string {
  if (typeof selection === "string") {
    return isValidHexColor(selection) ? selection : DEFAULT_ACCENT_VALUE;
  }

  if (selection.kind === "preset") {
    return (
      ACCENT_PRESETS.find((preset) => preset.id === selection.value)?.value ??
      DEFAULT_ACCENT_VALUE
    );
  }
  if (selection.kind === "windows") {
    return windowsAccentColor && isValidHexColor(windowsAccentColor)
      ? windowsAccentColor
      : DEFAULT_ACCENT_VALUE;
  }
  return isValidHexColor(selection.value) ? selection.value : DEFAULT_ACCENT_VALUE;
}

/** Convert a Windows ARGB/BGR DWORD to the CSS order #RRGGBB. */
export function windowsBgrDwordToCss(value: number): string {
  if (!Number.isInteger(value) || value < 0 || value > 0xffffffff) {
    throw new RangeError("Windows accent DWORD must be an unsigned 32-bit integer");
  }
  const red = value & 0xff;
  const green = (value >>> 8) & 0xff;
  const blue = (value >>> 16) & 0xff;
  return `#${toHex(red)}${toHex(green)}${toHex(blue)}`;
}

export function deriveAccentVariables(
  accent: string,
  colorMode: ColorMode = "light",
  prefersDark = false,
): AccentCssVariables {
  const resolved = isValidHexColor(accent) ? accent : DEFAULT_ACCENT_VALUE;
  const rgb = parseHexColor(resolved);
  const darkMode = resolveEffectiveColorMode(colorMode, prefersDark) === "dark";

  return {
    "--accent": resolved,
    "--accent-hover": rgbToHex(mixRgb(rgb, darkMode ? [255, 255, 255] : [0, 0, 0], 0.12)),
    "--accent-soft": rgbToHex(mixRgb(rgb, darkMode ? [0, 0, 0] : [255, 255, 255], darkMode ? 0.65 : 0.88)),
    "--focus": resolved,
    "--on-accent": readableAccentText(rgb),
    // 强调色文字（用于中性面 / accent-soft 上的可读文本）：向 ink 方向混合，
    // 保证任意自定义强调色都满足 WCAG AA（亮色向 #111827、暗色向 #E8EAEE 收敛）。
    "--accent-ink": rgbToHex(mixRgb(rgb, darkMode ? [232, 234, 238] : [17, 24, 39], 0.25)),
  };
}

/** Apply the theme attribute, the explicit accent variables and, when provided, appearance variables. */
export function applyTheme(
  root: HTMLElement,
  colorMode: ColorMode,
  accent?: AccentColor | string,
  windowsAccentColor?: string | null,
  prefersDark?: boolean,
  appearance?: AppearanceSettings,
): void {
  root.setAttribute("data-theme", colorMode);
  if (!root.style) {
    return;
  }

  if (accent !== undefined) {
    const variables = deriveAccentVariables(
      resolveAccentColor(accent, windowsAccentColor),
      colorMode,
      prefersDark ?? prefersDarkColorScheme(root),
    );
    for (const name of ACCENT_VARIABLE_NAMES) {
      root.style.setProperty(name, variables[name]);
    }
  }

  if (appearance) {
    const variables = deriveAppearanceVariables(
      appearance,
      resolveEffectiveColorMode(colorMode, prefersDark ?? prefersDarkColorScheme(root)),
    );
    for (const name of APPEARANCE_VARIABLE_NAMES) {
      const value = variables[name];
      if (value === "") {
        // 空栈 = 交还样式表默认（例如未自定义正文字体时）
        root.style.removeProperty(name);
      } else {
        root.style.setProperty(name, value);
      }
    }
  }
}

/** Capture the exact values touched by applyTheme so a draft preview is reversible. */
export function captureThemeState(root: HTMLElement): ThemeState {
  const variables = {} as Record<ThemeVariableName, string | null>;
  for (const name of THEME_VARIABLE_NAMES) {
    const value = root.style?.getPropertyValue(name) ?? "";
    variables[name] = value === "" ? null : value;
  }
  return {
    colorMode: root.getAttribute("data-theme"),
    variables,
  };
}

/** Restore only the state captured by captureThemeState. */
export function restoreThemeState(root: HTMLElement, state: ThemeState): void {
  if (state.colorMode === null) {
    root.removeAttribute("data-theme");
  } else {
    root.setAttribute("data-theme", state.colorMode);
  }

  for (const name of THEME_VARIABLE_NAMES) {
    const value = state.variables[name];
    if (value === null) {
      root.style?.removeProperty(name);
    } else {
      root.style?.setProperty(name, value);
    }
  }
}

/** Preview returns the previous state, allowing a caller to restore it on Cancel. */
export function previewTheme(
  root: HTMLElement,
  colorMode: ColorMode,
  accent?: AccentColor | string,
  windowsAccentColor?: string | null,
  appearance?: AppearanceSettings,
): ThemeState {
  const previous = captureThemeState(root);
  applyTheme(root, colorMode, accent, windowsAccentColor, undefined, appearance);
  return previous;
}

/** Returns a validation message, or null when the editor profile is valid. */
export function validateEditorProfile(profile: EditorProfile): string | null {
  if (profile.executable.trim() === "") {
    return null;
  }

  const placeholderCount = profile.arguments.reduce(
    (count, argument) => count + countLiteralOccurrences(argument, EDITOR_PATH_PLACEHOLDER),
    0,
  );
  if (placeholderCount !== 1) {
    return "editor.arguments must contain exactly one {path} placeholder when editor.executable is configured";
  }

  if (isBatchExecutable(profile.executable) && profile.arguments.some((argument) => argument.includes('"'))) {
    return "cmd.exe batch arguments cannot contain the double quote character";
  }
  return null;
}

export function isBatchExecutable(executable: string): boolean {
  const normalized = executable.trim().toLowerCase();
  return normalized.endsWith(".cmd") || normalized.endsWith(".bat");
}

function isAccentPresetId(value: string): value is AccentPresetId {
  return ACCENT_PRESETS.some((preset) => preset.id === value);
}

export function prefersDarkColorScheme(root: HTMLElement): boolean {
  const rootWindow = root.ownerDocument?.defaultView;
  if (rootWindow && typeof rootWindow.matchMedia === "function") {
    return rootWindow.matchMedia(PREFERS_DARK_QUERY).matches;
  }

  const globalScope = globalThis as typeof globalThis & {
    matchMedia?: (query: string) => MediaQueryList;
  };
  if (typeof globalScope.matchMedia === "function") {
    return globalScope.matchMedia(PREFERS_DARK_QUERY).matches;
  }

  return false;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function hasExactKeys(value: Record<string, unknown>, keys: string[]): boolean {
  const actual = Object.keys(value).sort();
  return actual.length === keys.length && actual.every((key, index) => key === keys.sort()[index]);
}

function countLiteralOccurrences(value: string, literal: string): number {
  let count = 0;
  let offset = 0;
  while (true) {
    const index = value.indexOf(literal, offset);
    if (index < 0) {
      return count;
    }
    count += 1;
    offset = index + literal.length;
  }
}

function parseHexColor(value: string): [number, number, number] {
  return [
    Number.parseInt(value.slice(1, 3), 16),
    Number.parseInt(value.slice(3, 5), 16),
    Number.parseInt(value.slice(5, 7), 16),
  ];
}

function mixRgb(
  source: [number, number, number],
  target: [number, number, number],
  amount: number,
): [number, number, number] {
  return source.map((channel, index) => Math.round(channel + (target[index] - channel) * amount)) as [
    number,
    number,
    number
  ];
}

function rgbToHex(rgb: [number, number, number]): string {
  return `#${toHex(rgb[0])}${toHex(rgb[1])}${toHex(rgb[2])}`;
}

function toHex(value: number): string {
  return value.toString(16).padStart(2, "0").toUpperCase();
}

function readableAccentText(rgb: [number, number, number]): string {
  const whiteContrast = contrastRatio(rgb, [255, 255, 255]);
  const darkContrast = contrastRatio(rgb, [17, 24, 39]);
  return whiteContrast >= darkContrast ? LIGHT_TEXT : DARK_TEXT;
}

function contrastRatio(first: [number, number, number], second: [number, number, number]): number {
  const firstLuminance = relativeLuminance(first);
  const secondLuminance = relativeLuminance(second);
  const lighter = Math.max(firstLuminance, secondLuminance);
  const darker = Math.min(firstLuminance, secondLuminance);
  return (lighter + 0.05) / (darker + 0.05);
}

function relativeLuminance(rgb: [number, number, number]): number {
  const channels = rgb.map((channel) => {
    const normalized = channel / 255;
    return normalized <= 0.03928
      ? normalized / 12.92
      : ((normalized + 0.055) / 1.055) ** 2.4;
  });
  return channels[0] * 0.2126 + channels[1] * 0.7152 + channels[2] * 0.0722;
}
