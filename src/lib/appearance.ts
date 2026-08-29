// Appearance 设置域（settings v3）：风格预设包、字体栈清洗、CSS 变量派生。
// 纯逻辑模块（参照 theme.ts 模式）；类型与 api.ts 仅做 type-only 互引，无运行时循环。

import type { AccentPresetId } from "./api";

export type Density = "compact" | "comfortable";

export type StylePresetId = "windy" | "cloud" | "ink" | "midnight";

export interface NeutralSet {
  bg: string;
  surface: string;
  sunken: string;
  line: string;
  ink: string;
  muted: string;
}

/** settings v3 `appearance` 域；中性色始终物化保存（另存自定义主题时不丢失）。 */
export interface AppearanceSettings {
  stylePreset: StylePresetId | null;
  /** 0..=20 px。 */
  radius: number;
  /** 13..=16 px。 */
  fontSize: number;
  density: Density;
  /** "" = 使用样式表默认系统栈；否则为用户自定义 font stack。 */
  fontFamily: string;
  neutrals: { light: NeutralSet; dark: NeutralSet };
}

export const STYLE_PRESET_IDS: ReadonlyArray<StylePresetId> = [
  "windy",
  "cloud",
  "ink",
  "midnight",
];

export const MIN_RADIUS = 0;
export const MAX_RADIUS = 20;
export const MIN_FONT_SIZE = 13;
export const MAX_FONT_SIZE = 16;

/** Windy 默认中性色（与 App.css :root 保持一致）。 */
export const WINDY_NEUTRALS: AppearanceSettings["neutrals"] = {
  light: {
    bg: "#f5f6f8",
    surface: "#ffffff",
    sunken: "#eceef2",
    line: "#dfe3e8",
    ink: "#17191f",
    muted: "#6a7280",
  },
  dark: {
    bg: "#121417",
    surface: "#1a1d22",
    sunken: "#23272e",
    line: "#2e343c",
    ink: "#e8eaee",
    muted: "#949ca8",
  },
};

export interface StylePreset {
  readonly id: StylePresetId;
  readonly label: string;
  readonly description: string;
  /** 推荐强调色（六个稳定预设 id 之一）。 */
  readonly accent: AccentPresetId;
  readonly radius: number;
  readonly fontSize: number;
  readonly density: Density;
  readonly neutrals: AppearanceSettings["neutrals"];
}

/** 内置风格预设包：完整 token 组合（中性色 + 推荐强调色 + 圆角 / 密度）。 */
export const STYLE_PRESETS: ReadonlyArray<StylePreset> = [
  {
    id: "windy",
    label: "Windy default",
    description: "The calm teal baseline.",
    accent: "windy-teal",
    radius: 10,
    fontSize: 14,
    density: "comfortable",
    neutrals: WINDY_NEUTRALS,
  },
  {
    id: "cloud",
    label: "Cloud",
    description: "Cool gray field, warm coral accent.",
    accent: "coral",
    radius: 12,
    fontSize: 14,
    density: "comfortable",
    neutrals: {
      light: {
        bg: "#eef0f3",
        surface: "#ffffff",
        sunken: "#e3e7ec",
        line: "#d8dde4",
        ink: "#1d2129",
        muted: "#6b7280",
      },
      dark: {
        bg: "#101216",
        surface: "#171a20",
        sunken: "#21252d",
        line: "#2b313a",
        ink: "#e7e9ee",
        muted: "#8f97a3",
      },
    },
  },
  {
    id: "ink",
    label: "Ink wash",
    description: "Warm paper neutrals with a violet accent.",
    accent: "violet",
    radius: 6,
    fontSize: 14,
    density: "comfortable",
    neutrals: {
      light: {
        bg: "#f4f2ee",
        surface: "#fbfaf7",
        sunken: "#eae7e0",
        line: "#ddd9cf",
        ink: "#26231e",
        muted: "#6f6a5f",
      },
      dark: {
        bg: "#191713",
        surface: "#211e18",
        sunken: "#2a2620",
        line: "#37322a",
        ink: "#e9e6de",
        muted: "#a09a8c",
      },
    },
  },
  {
    id: "midnight",
    label: "Midnight",
    description: "Deep blue-black night with an ocean accent.",
    accent: "ocean-blue",
    radius: 14,
    fontSize: 14,
    density: "comfortable",
    neutrals: {
      light: {
        bg: "#eef1f6",
        surface: "#ffffff",
        sunken: "#e2e8f0",
        line: "#d5dce6",
        ink: "#131b2a",
        muted: "#5d6b80",
      },
      dark: {
        bg: "#0c1018",
        surface: "#131926",
        sunken: "#1b2333",
        line: "#263046",
        ink: "#e4e9f4",
        muted: "#8b97ad",
      },
    },
  },
];

export const DEFAULT_APPEARANCE: AppearanceSettings = {
  stylePreset: "windy",
  radius: 10,
  fontSize: 14,
  density: "comfortable",
  fontFamily: "",
  neutrals: WINDY_NEUTRALS,
};

export interface BodyFontOption {
  readonly id: string;
  readonly label: string;
  /** CSS font stack；"" 表示样式表默认。 */
  readonly stack: string;
}

/** 正文字体预设：每项以自身字体渲染预览；自定义栈走输入框。 */
export const BODY_FONT_OPTIONS: ReadonlyArray<BodyFontOption> = [
  { id: "system", label: "System default", stack: "" },
  {
    id: "segoe-variable",
    label: "Segoe UI Variable",
    stack: '"Segoe UI Variable", "Segoe UI", system-ui, "Microsoft YaHei", sans-serif',
  },
  {
    id: "segoe-ui",
    label: "Segoe UI",
    stack: '"Segoe UI", system-ui, "Microsoft YaHei", sans-serif',
  },
  { id: "verdana", label: "Verdana", stack: 'Verdana, Tahoma, sans-serif' },
  { id: "georgia", label: "Georgia", stack: 'Georgia, "Times New Roman", serif' },
  { id: "cascadia", label: "Cascadia Code", stack: '"Cascadia Code", Consolas, monospace' },
];

/** 清洗自定义字体栈：控制字符折叠为空格、去首尾空白；空串 = 回退默认栈。 */
export function sanitizeFontFamily(value: string): string {
  const cleaned = value.replace(/[\u0000-\u001f\u007f]+/g, " ").trim();
  return cleaned.length > 200 ? cleaned.slice(0, 200) : cleaned;
}

export function isValidStylePresetId(value: string): value is StylePresetId {
  return STYLE_PRESET_IDS.some((id) => id === value);
}

function isDensity(value: unknown): value is Density {
  return value === "compact" || value === "comfortable";
}

function isNeutralSet(value: unknown): value is NeutralSet {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const record = value as Record<string, unknown>;
  return (
    typeof record.bg === "string" &&
    typeof record.surface === "string" &&
    typeof record.sunken === "string" &&
    typeof record.line === "string" &&
    typeof record.ink === "string" &&
    typeof record.muted === "string"
  );
}

/**
 * 严格解析 appearance 域（与 Rust 侧校验口径一致）；不合法返回 null，由调用方回退默认。
 */
export function parseAppearance(value: unknown): AppearanceSettings | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return null;
  }
  const record = value as Record<string, unknown>;
  if (
    record.stylePreset !== null &&
    (typeof record.stylePreset !== "string" || !isValidStylePresetId(record.stylePreset))
  ) {
    return null;
  }
  if (
    typeof record.radius !== "number" ||
    !Number.isInteger(record.radius) ||
    record.radius < MIN_RADIUS ||
    record.radius > MAX_RADIUS
  ) {
    return null;
  }
  if (
    typeof record.fontSize !== "number" ||
    !Number.isInteger(record.fontSize) ||
    record.fontSize < MIN_FONT_SIZE ||
    record.fontSize > MAX_FONT_SIZE
  ) {
    return null;
  }
  if (!isDensity(record.density) || typeof record.fontFamily !== "string") {
    return null;
  }
  if (/[\u0000-\u001f\u007f]/.test(record.fontFamily) || record.fontFamily.length > 200) {
    return null;
  }
  if (
    typeof record.neutrals !== "object" ||
    record.neutrals === null ||
    !isNeutralSet((record.neutrals as { light?: unknown }).light) ||
    !isNeutralSet((record.neutrals as { dark?: unknown }).dark)
  ) {
    return null;
  }
  const neutrals = record.neutrals as { light: NeutralSet; dark: NeutralSet };
  return {
    stylePreset: record.stylePreset as StylePresetId | null,
    radius: record.radius,
    fontSize: record.fontSize,
    density: record.density,
    fontFamily: record.fontFamily.trim(),
    neutrals: { light: { ...neutrals.light }, dark: { ...neutrals.dark } },
  };
}

/** 由预设物化一份 appearance（选中预设包时连同推荐强调色一起应用）。 */
export function materializePreset(preset: StylePreset): AppearanceSettings {
  return {
    stylePreset: preset.id,
    radius: preset.radius,
    fontSize: preset.fontSize,
    density: preset.density,
    fontFamily: "",
    neutrals: {
      light: { ...preset.neutrals.light },
      dark: { ...preset.neutrals.dark },
    },
  };
}

export const APPEARANCE_VARIABLE_NAMES = [
  "--bg",
  "--surface",
  "--sunken",
  "--line",
  "--ink",
  "--muted",
  "--app-radius",
  "--app-font-size",
  "--app-density",
  "--font-body",
] as const;

export type AppearanceCssVariables = Record<
  (typeof APPEARANCE_VARIABLE_NAMES)[number],
  string
>;

export type EffectiveMode = "light" | "dark";

/** 派生 appearance 的全部 CSS 变量；--font-body 为空串表示交还样式表默认（应用时移除内联值）。 */
export function deriveAppearanceVariables(
  appearance: AppearanceSettings,
  effectiveMode: EffectiveMode,
): AppearanceCssVariables {
  const neutrals = appearance.neutrals[effectiveMode];
  return {
    "--bg": neutrals.bg,
    "--surface": neutrals.surface,
    "--sunken": neutrals.sunken,
    "--line": neutrals.line,
    "--ink": neutrals.ink,
    "--muted": neutrals.muted,
    "--app-radius": `${appearance.radius}px`,
    "--app-font-size": `${appearance.fontSize}px`,
    "--app-density": appearance.density === "compact" ? "0.85" : "1",
    "--font-body": sanitizeFontFamily(appearance.fontFamily),
  };
}
