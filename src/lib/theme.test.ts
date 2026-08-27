import { describe, expect, it, vi } from "vitest";
import {
  ACCENT_PRESETS,
  DEFAULT_ACCENT_COLOR,
  DEFAULT_EDITOR_ARGUMENTS,
  applyTheme,
  captureThemeState,
  deriveAccentVariables,
  isValidColorMode,
  isValidHexColor,
  parseAccentColor,
  previewTheme,
  resolveAccentColor,
  restoreThemeState,
  resolveEffectiveColorMode,
  selectAccentColor,
  isValidTheme,
  validateEditorProfile,
  windowsBgrDwordToCss,
  type EditorProfile,
} from "./theme";

describe("accent presets", () => {
  it("exports the six stable ids and exact CSS values", () => {
    expect(ACCENT_PRESETS).toEqual([
      { id: "windy-teal", label: "Windy teal", value: "#0E7D8C" },
      { id: "ocean-blue", label: "Ocean blue", value: "#2563EB" },
      { id: "violet", label: "Violet", value: "#7C3AED" },
      { id: "amber", label: "Amber", value: "#D97706" },
      { id: "coral", label: "Coral", value: "#E05A47" },
      { id: "rose", label: "Rose", value: "#D94675" },
    ]);
  });

  it("resolves preset, custom, and Windows selections with teal fallback", () => {
    expect(resolveAccentColor({ kind: "preset", value: "ocean-blue" })).toBe("#2563EB");
    expect(resolveAccentColor({ kind: "custom", value: "#12abEF" })).toBe("#12abEF");
    expect(resolveAccentColor({ kind: "windows" }, "#AABBCC")).toBe("#AABBCC");
    expect(resolveAccentColor({ kind: "windows" }, "not-a-color")).toBe("#0E7D8C");
    expect(resolveAccentColor({ kind: "windows" })).toBe("#0E7D8C");
  });

  it("parses only the v2 tagged union", () => {
    expect(parseAccentColor({ kind: "preset", value: "rose" })).toEqual({
      kind: "preset",
      value: "rose",
    });
    expect(parseAccentColor({ kind: "windows" })).toEqual({ kind: "windows" });
    expect(parseAccentColor({ kind: "custom", value: "#ABCDEF" })).toEqual({
      kind: "custom",
      value: "#ABCDEF",
    });
    expect(parseAccentColor({ kind: "custom", value: "#ABCDE" })).toBeNull();
    expect(parseAccentColor({ kind: "windows", value: "#123456" })).toBeNull();
    expect(parseAccentColor({ kind: "preset", value: "blue" })).toBeNull();
  });

  it("creates only valid selections", () => {
    expect(selectAccentColor("preset", "violet")).toEqual({
      kind: "preset",
      value: "violet",
    });
    expect(selectAccentColor("windows")).toEqual({ kind: "windows" });
    expect(selectAccentColor("custom", "#123456")).toEqual({
      kind: "custom",
      value: "#123456",
    });
    expect(selectAccentColor("windows", "#123456")).toBeNull();
  });
});

describe("color mode and hex validation", () => {
  it("accepts exactly the three color modes", () => {
    expect(["system", "light", "dark"].map(isValidColorMode)).toEqual([true, true, true]);
    expect(isValidTheme("dark")).toBe(true);
    expect(isValidColorMode("Light")).toBe(false);
    expect(isValidColorMode("midnight")).toBe(false);
  });

  it("accepts strict #RRGGBB values and rejects nearby forms", () => {
    expect(isValidHexColor("#000000")).toBe(true);
    expect(isValidHexColor("#aBcD09")).toBe(true);
    expect(isValidHexColor("#12345")).toBe(false);
    expect(isValidHexColor("#1234567")).toBe(false);
    expect(isValidHexColor("123456")).toBe(false);
    expect(isValidHexColor("#12-456")).toBe(false);
    expect(isValidHexColor(" #123456")).toBe(false);
  });
});

describe("Windows accent conversion", () => {
  it("converts the low BGR bytes from an ARGB DWORD", () => {
    expect(windowsBgrDwordToCss(0xff534c2a)).toBe("#2A4C53");
    expect(windowsBgrDwordToCss(0x00112233)).toBe("#332211");
  });
});

describe("derived accent variables", () => {
  it("derives all explicit CSS variables and a readable foreground", () => {
    const variables = deriveAccentVariables("#0E7D8C", "light");

    expect(variables).toEqual({
      "--accent": "#0E7D8C",
      "--accent-hover": expect.stringMatching(/^#[0-9A-F]{6}$/),
      "--accent-soft": expect.stringMatching(/^#[0-9A-F]{6}$/),
      "--focus": "#0E7D8C",
      "--on-accent": "#FFFFFF",
    });
    expect(variables["--accent-hover"]).not.toBe(variables["--accent"]);
    expect(deriveAccentVariables("#D97706", "light")["--on-accent"]).toBe("#111827");
  });

  it("uses the effective light or dark formula for system mode", () => {
    expect(resolveEffectiveColorMode("system", false)).toBe("light");
    expect(resolveEffectiveColorMode("system", true)).toBe("dark");

    const systemLight = deriveAccentVariables("#0E7D8C", "system", false);
    const systemDark = deriveAccentVariables("#0E7D8C", "system", true);

    expect(systemLight["--accent-hover"]).toBe("#0C6E7B");
    expect(systemLight["--accent-soft"]).toBe("#E2EFF1");
    expect(systemDark["--accent-hover"]).toBe("#2B8D9A");
    expect(systemDark["--accent-soft"]).toBe("#052C31");
    expect(systemDark).not.toEqual(systemLight);
  });
});

describe("system theme media resolution", () => {
  it("reads prefers-color-scheme from the root window before deriving variables", () => {
    const properties = new Map<string, string>();
    const matchMedia = vi.fn(() => ({ matches: true }) as MediaQueryList);
    const root = {
      ownerDocument: { defaultView: { matchMedia } },
      style: {
        setProperty(name: string, value: string) {
          properties.set(name, value);
        },
      },
      setAttribute() {},
    } as unknown as HTMLElement;

    applyTheme(root, "system", DEFAULT_ACCENT_COLOR);

    expect(matchMedia).toHaveBeenCalledWith("(prefers-color-scheme: dark)");
    expect(properties.get("--accent-hover")).toBe("#2B8D9A");
    expect(properties.get("--accent-soft")).toBe("#052C31");
  });

  it("falls back to the light formula when matchMedia is unavailable", () => {
    vi.stubGlobal("matchMedia", undefined);
    try {
      const properties = new Map<string, string>();
      const root = {
        style: {
          setProperty(name: string, value: string) {
            properties.set(name, value);
          },
        },
        setAttribute() {},
      } as unknown as HTMLElement;

      applyTheme(root, "system", DEFAULT_ACCENT_COLOR);

      expect(properties.get("--accent-hover")).toBe("#0C6E7B");
      expect(properties.get("--accent-soft")).toBe("#E2EFF1");
    } finally {
      vi.unstubAllGlobals();
    }
  });
});

describe("reversible theme preview", () => {
  it("changes only data-theme and the five accent variables, then restores them", () => {
    const attributes = new Map<string, string>();
    const properties = new Map<string, string>();
    const root = {
      style: {
        setProperty(name: string, value: string) {
          properties.set(name, value);
        },
        getPropertyValue(name: string) {
          return properties.get(name) ?? "";
        },
        removeProperty(name: string) {
          properties.delete(name);
        },
      },
      setAttribute(name: string, value: string) {
        attributes.set(name, value);
      },
      getAttribute(name: string) {
        return attributes.get(name) ?? null;
      },
      removeAttribute(name: string) {
        attributes.delete(name);
      },
    } as unknown as HTMLElement;

    applyTheme(root, "light", DEFAULT_ACCENT_COLOR);
    const saved = captureThemeState(root);
    previewTheme(root, "dark", { kind: "preset", value: "rose" });

    expect(attributes.get("data-theme")).toBe("dark");
    expect(properties.get("--accent")).toBe("#D94675");
    expect(properties.has("--accent-hover")).toBe(true);
    expect([...properties.keys()].sort()).toEqual([
      "--accent",
      "--accent-hover",
      "--accent-soft",
      "--focus",
      "--on-accent",
    ]);

    restoreThemeState(root, saved);
    expect(attributes.get("data-theme")).toBe("light");
    expect(properties.get("--accent")).toBe("#0E7D8C");
  });
});

describe("editor profile validation", () => {
  const configured = (arguments_: string[]): EditorProfile => ({
    executable: "C:/Tools/Code.exe",
    arguments: arguments_,
  });

  it("requires exactly one literal {path} for configured editors", () => {
    expect(validateEditorProfile(configured(["--reuse-window"]))).toMatch(/exactly one/);
    expect(validateEditorProfile(configured(["{path}", "--folder", "{path}"]))).toMatch(
      /exactly one/,
    );
    expect(validateEditorProfile(configured(["--folder", "{path}"]))).toBeNull();
  });

  it("allows empty executable as the unconfigured default", () => {
    expect(DEFAULT_EDITOR_ARGUMENTS).toEqual(["{path}"]);
    expect(validateEditorProfile({ executable: "", arguments: [] })).toBeNull();
    expect(validateEditorProfile({ executable: "  ", arguments: ["{path}", "{path}"] })).toBeNull();
  });

  it("keeps batch quote validation explicit", () => {
    expect(validateEditorProfile({ executable: "open.cmd", arguments: ['--title="x"', "{path}"] })).toMatch(
      /double quote/,
    );
  });
});
