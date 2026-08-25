// 主题纯逻辑测试（D1 / D6，D12：只测纯函数）。

import { describe, expect, it } from "vitest";
import { EDITOR_PRESETS, isValidTheme } from "./theme";

describe("isValidTheme", () => {
  it("accepts the three supported values", () => {
    expect(isValidTheme("system")).toBe(true);
    expect(isValidTheme("light")).toBe(true);
    expect(isValidTheme("dark")).toBe(true);
  });

  it("rejects unknown or empty values", () => {
    expect(isValidTheme("midnight")).toBe(false);
    expect(isValidTheme("")).toBe(false);
    expect(isValidTheme("Light")).toBe(false);
    expect(isValidTheme("  ")).toBe(false);
  });
});

describe("EDITOR_PRESETS", () => {
  it("covers the D6 presets code / code-insiders / cursor", () => {
    const commands = EDITOR_PRESETS.map((p) => p.command);
    expect(commands).toContain("code");
    expect(commands).toContain("code-insiders");
    expect(commands).toContain("cursor");
  });

  it("has non-empty labels and stable command keys", () => {
    for (const p of EDITOR_PRESETS) {
      expect(p.label.trim().length).toBeGreaterThan(0);
      expect(p.command.trim().length).toBeGreaterThan(0);
    }
  });
});