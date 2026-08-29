import { describe, expect, it } from "vitest";
import {
  APPEARANCE_VARIABLE_NAMES,
  BODY_FONT_OPTIONS,
  DEFAULT_APPEARANCE,
  STYLE_PRESETS,
  WINDY_NEUTRALS,
  deriveAppearanceVariables,
  materializePreset,
  parseAppearance,
  sanitizeFontFamily,
} from "./appearance";
import { deriveAccentVariables } from "./theme";

const HEX = /^#[0-9A-Fa-f]{6}$/;

describe("style presets", () => {
  it("ships exactly the four built-in packs with stable ids", () => {
    expect(STYLE_PRESETS.map((preset) => preset.id)).toEqual([
      "windy",
      "cloud",
      "ink",
      "midnight",
    ]);
  });

  it("keeps every neutral value a strict #RRGGBB color", () => {
    for (const preset of STYLE_PRESETS) {
      for (const set of [preset.neutrals.light, preset.neutrals.dark]) {
        for (const value of Object.values(set)) {
          expect(value, `${preset.id} neutral ${value}`).toMatch(HEX);
        }
      }
    }
    for (const set of [WINDY_NEUTRALS.light, WINDY_NEUTRALS.dark]) {
      for (const value of Object.values(set)) {
        expect(value).toMatch(HEX);
      }
    }
  });

  it("keeps preset radii and font sizes inside the persisted ranges", () => {
    for (const preset of STYLE_PRESETS) {
      expect(preset.radius).toBeGreaterThanOrEqual(0);
      expect(preset.radius).toBeLessThanOrEqual(20);
      expect(preset.fontSize).toBeGreaterThanOrEqual(13);
      expect(preset.fontSize).toBeLessThanOrEqual(16);
    }
  });

  it("materializes a preset into a full appearance record", () => {
    const cloud = STYLE_PRESETS.find((preset) => preset.id === "cloud")!;
    const materialized = materializePreset(cloud);
    expect(materialized).toEqual({
      stylePreset: "cloud",
      radius: cloud.radius,
      fontSize: cloud.fontSize,
      density: cloud.density,
      fontFamily: "",
      neutrals: cloud.neutrals,
    });
    expect(materializePreset(STYLE_PRESETS[0])).not.toBe(DEFAULT_APPEARANCE);
    expect(materializePreset(STYLE_PRESETS[0])).toEqual(DEFAULT_APPEARANCE);
  });
});

describe("sanitizeFontFamily", () => {
  it("collapses control character runs and trims whitespace", () => {
    expect(sanitizeFontFamily("  Segoe UI\t\n, serif  ")).toBe("Segoe UI , serif");
    expect(sanitizeFontFamily('Georgia, "Times New Roman", serif')).toBe(
      'Georgia, "Times New Roman", serif',
    );
  });

  it("returns an empty stack for blank input", () => {
    expect(sanitizeFontFamily("   ")).toBe("");
  });

  it("caps the stack at 200 characters", () => {
    const long = "A".repeat(300);
    expect(sanitizeFontFamily(long)).toHaveLength(200);
  });
});

describe("parseAppearance", () => {
  it("accepts the default record and normalizes fontFamily", () => {
    expect(parseAppearance(DEFAULT_APPEARANCE)).toEqual(DEFAULT_APPEARANCE);
    expect(
      parseAppearance({ ...DEFAULT_APPEARANCE, fontFamily: "  Segoe UI  " }),
    ).toEqual({ ...DEFAULT_APPEARANCE, fontFamily: "Segoe UI" });
  });

  it("rejects out-of-range or malformed fields", () => {
    expect(parseAppearance({ ...DEFAULT_APPEARANCE, radius: 21 })).toBeNull();
    expect(parseAppearance({ ...DEFAULT_APPEARANCE, radius: -1 })).toBeNull();
    expect(parseAppearance({ ...DEFAULT_APPEARANCE, fontSize: 12 })).toBeNull();
    expect(parseAppearance({ ...DEFAULT_APPEARANCE, fontSize: 17 })).toBeNull();
    expect(parseAppearance({ ...DEFAULT_APPEARANCE, density: "cozy" })).toBeNull();
    expect(parseAppearance({ ...DEFAULT_APPEARANCE, stylePreset: "paper" })).toBeNull();
    expect(parseAppearance({ ...DEFAULT_APPEARANCE, fontFamily: "bad\nfont" })).toBeNull();
    expect(parseAppearance({ ...DEFAULT_APPEARANCE, neutrals: null })).toBeNull();
    expect(parseAppearance(null)).toBeNull();
    expect(parseAppearance("appearance")).toBeNull();
  });

  it("accepts a fully custom theme (null stylePreset)", () => {
    const custom = {
      stylePreset: null,
      radius: 0,
      fontSize: 16,
      density: "compact",
      fontFamily: '"Cascadia Code", monospace',
      neutrals: {
        light: { bg: "#010203", surface: "#040506", sunken: "#070809", line: "#0a0b0c", ink: "#0d0e0f", muted: "#101112" },
        dark: { bg: "#111213", surface: "#141516", sunken: "#171819", line: "#1a1b1c", ink: "#1d1e1f", muted: "#202122" },
      },
    } as const;
    expect(parseAppearance(custom)).toEqual(custom);
  });
});

describe("deriveAppearanceVariables", () => {
  it("derives all ten variables from the active mode", () => {
    const light = deriveAppearanceVariables(DEFAULT_APPEARANCE, "light");
    const dark = deriveAppearanceVariables(DEFAULT_APPEARANCE, "dark");

    expect(Object.keys(light).sort()).toEqual([...APPEARANCE_VARIABLE_NAMES].sort());
    expect(light["--bg"]).toBe("#f5f6f8");
    expect(light["--app-radius"]).toBe("10px");
    expect(light["--app-font-size"]).toBe("14px");
    expect(light["--app-density"]).toBe("1");
    expect(light["--font-body"]).toBe("");
    expect(dark["--bg"]).toBe("#121417");
    expect(dark["--surface"]).toBe("#1a1d22");
  });

  it("switches neutrals with the mode and encodes compact density", () => {
    const ink = STYLE_PRESETS.find((preset) => preset.id === "ink")!;
    const appearance = { ...materializePreset(ink), density: "compact" as const };
    const dark = deriveAppearanceVariables(appearance, "dark");
    expect(dark["--bg"]).toBe(ink.neutrals.dark.bg);
    expect(dark["--app-density"]).toBe("0.85");
    expect(dark["--app-radius"]).toBe(`${ink.radius}px`);
  });

  it("keeps accent derivation independent from appearance neutrals", () => {
    const accent = deriveAccentVariables("#D97706", "light");
    const appearance = deriveAppearanceVariables(DEFAULT_APPEARANCE, "light");
    expect(accent["--accent"]).toBe("#D97706");
    expect(appearance["--ink"]).toBe("#17191f");
  });
});

describe("body font options", () => {
  it("starts with the system default empty stack and keeps unique ids", () => {
    expect(BODY_FONT_OPTIONS[0]).toEqual({ id: "system", label: "System default", stack: "" });
    const ids = BODY_FONT_OPTIONS.map((option) => option.id);
    expect(new Set(ids).size).toBe(ids.length);
    for (const option of BODY_FONT_OPTIONS) {
      expect(option.stack === "" || /".+"/.test(option.stack) || /^[A-Za-z]/.test(option.stack)).toBe(
        true,
      );
    }
  });
});
