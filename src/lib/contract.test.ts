// 跨语言常量契约测试（A4）：Rust 与 TypeScript 各持一份默认值 / 预设 id，
// 用源码扫描断言两边**相等**（双向），让「靠注释保持一致」升级为「靠测试保持一致」。
//
// 这是全仓库唯一有意使用的源码扫描测试：跨语言常量无法共享代码，
// 源码扫描是这里最轻且有效的护栏（其余场景不推广，见 README 远期规划 A6）。

import { describe, expect, it } from "vitest";
// @ts-expect-error The application intentionally does not ship Node typings; this is a Vitest-only static contract test.
import { existsSync, readFileSync } from "node:fs";
// @ts-expect-error The application intentionally does not ship Node typings; this is a Vitest-only static contract test.
import { resolve } from "node:path";
import { STYLE_PRESET_IDS, WINDY_NEUTRALS } from "./appearance";
import { ACCENT_PRESETS } from "./theme";

declare const process: { cwd: () => string };

const readSource = (relativePath: string) => {
  const absolutePath = resolve(process.cwd(), relativePath);
  return existsSync(absolutePath) ? readFileSync(absolutePath, "utf8") : "";
};

const settingsSource = readSource("src-tauri/src/project/settings.rs");

function functionBody(source: string, name: string): string {
  const start = source.indexOf(`fn ${name}(`);
  if (start < 0) {
    return "";
  }
  const open = source.indexOf("{", start);
  let depth = 0;
  for (let i = open; i < source.length; i += 1) {
    if (source[i] === "{") {
      depth += 1;
    } else if (source[i] === "}") {
      depth -= 1;
      if (depth === 0) {
        return source.slice(open, i + 1);
      }
    }
  }
  return "";
}

function constBlock(source: string, name: string): string {
  const start = source.indexOf(`const ${name}`);
  if (start < 0) {
    return "";
  }
  // 越过类型注解中的分号（如 `[&str; 4]`），从 `=` 之后再找语句结尾。
  const equals = source.indexOf("=", start);
  if (equals < 0) {
    return source.slice(start);
  }
  const end = source.indexOf(";", equals);
  return end < 0 ? source.slice(start) : source.slice(start, end + 1);
}

/** 取出代码块中所有 `"#rrggbb"` 形式的字面量（小写）。 */
function rustHexLiterals(block: string): string[] {
  return [...block.matchAll(/"(#[0-9a-fA-F]{6})"/g)].map((match) => match[1].toLowerCase());
}

/** 取出代码块中所有 `"identifier"` 形式的字面量（排除颜色值）。 */
function rustIdentifierLiterals(block: string): string[] {
  return [...block.matchAll(/"([a-z0-9][a-z0-9-]*)"/g)].map((match) => match[1]);
}

const sorted = (values: string[]) => [...values].sort();

describe("cross-language constant contract", () => {
  it("ships the Rust settings source for comparison", () => {
    expect(settingsSource.length).toBeGreaterThan(0);
  });

  it("keeps Windy neutrals equal to the Rust defaults (both directions)", () => {
    for (const mode of ["light", "dark"] as const) {
      const block = functionBody(settingsSource, `default_${mode}_neutrals`);
      const rustValues = rustHexLiterals(block);
      const tsValues = Object.values(WINDY_NEUTRALS[mode]).map((value) => value.toLowerCase());
      expect(sorted(rustValues), `${mode} neutrals`).toEqual(sorted(tsValues));
    }
  });

  it("keeps style preset ids equal to the Rust STYLE_PRESETS (both directions)", () => {
    const block = constBlock(settingsSource, "STYLE_PRESETS");
    expect(sorted(rustIdentifierLiterals(block))).toEqual(sorted([...STYLE_PRESET_IDS]));
  });

  it("keeps accent preset ids equal to the Rust ACCENT_PRESETS (both directions)", () => {
    const block = constBlock(settingsSource, "ACCENT_PRESETS");
    expect(sorted(rustIdentifierLiterals(block))).toEqual(sorted(ACCENT_PRESETS.map((p) => p.id)));
  });

  it("declares the frontend default settings exactly once", () => {
    const app = readSource("src/App.tsx");
    const dialog = readSource("src/components/SettingsDialog.tsx");
    expect(app).not.toMatch(/const DEFAULT_SETTINGS/);
    expect(dialog).not.toMatch(/const DEFAULT_SETTINGS/);
  });
});
