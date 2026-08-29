import { describe, expect, it } from "vitest";
// @ts-expect-error The application intentionally does not ship Node typings; this is a Vitest-only static contract test.
import { existsSync, readFileSync } from "node:fs";
// @ts-expect-error The application intentionally does not ship Node typings; this is a Vitest-only static contract test.
import { resolve } from "node:path";

declare const process: { cwd: () => string };

const readSource = (relativePath: string) => {
  const absolutePath = resolve(process.cwd(), relativePath);
  return existsSync(absolutePath) ? readFileSync(absolutePath, "utf8") : "";
};

const iconNames = [
  "windy",
  "layout-grid",
  "tag",
  "settings",
  "search",
  "plus",
  "folder-open",
  "play",
  "hammer",
  "more-horizontal",
  "code",
  "pencil",
  "trash",
  "refresh",
  "arrow-left",
  "close",
  "file-script",
  "ban",
  "git-branch",
  "check-circle",
  "alert-triangle",
  "help-circle",
  "activity",
  "overview",
  "layers",
  "history",
  "zap",
  "palette",
  "sun",
  "moon",
  "monitor",
  "info",
  "rotate-ccw",
  "chevron-left",
  "chevron-right",
  "chevron-down",
  "text",
  "align-left",
  "terminal",
  "folder-plus",
  "package",
] as const;

describe("custom SVG icon contract", () => {
  const icon = readSource("src/components/Icon.tsx");

  it("defines the complete semantic registry on a round 24px SVG canvas", () => {
    expect(icon).toContain("export type IconName");
    expect(icon).toContain('viewBox="0 0 24 24"');
    expect(icon).toContain('stroke="currentColor"');
    expect(icon).toContain('strokeLinecap="round"');
    expect(icon).toContain('strokeLinejoin="round"');
    expect(icon).toContain("title?: string");
    expect(icon).toContain("aria-hidden={title ? undefined : true}");

    for (const name of iconNames) {
      expect(icon, `missing icon name ${name}`).toContain(`"${name}"`);
    }
  });

  it("uses the icon layer across every visible UI surface", () => {
    const sources = [
      readSource("src/App.tsx"),
      readSource("src/components/Sidebar.tsx"),
      readSource("src/components/ProjectCard.tsx"),
      readSource("src/components/Modal.tsx"),
      readSource("src/components/AddProjectDialog.tsx"),
      readSource("src/components/EditProjectDialog.tsx"),
      readSource("src/components/ProjectFields.tsx"),
      readSource("src/components/SettingsDialog.tsx"),
      readSource("src/components/SettingsAppearancePanel.tsx"),
      readSource("src/components/SettingsEditorPanel.tsx"),
      readSource("src/components/SettingsGeneralPanel.tsx"),
      readSource("src/pages/ProjectDetail.tsx"),
    ];

    for (const source of sources) {
      expect(source).toContain("Icon");
    }
  });

  it("keeps compact icon-only controls accessible and removes text glyph stand-ins", () => {
    const app = readSource("src/App.tsx");
    const card = readSource("src/components/ProjectCard.tsx");
    const modal = readSource("src/components/Modal.tsx");
    const detail = readSource("src/pages/ProjectDetail.tsx");

    expect(app).toContain('aria-label="Dismiss"');
    expect(card).toContain('aria-label="More"');
    expect(modal).toContain('aria-label="Close"');
    expect(app).not.toContain("✕");
    expect(card).not.toContain("⋯");
    expect(detail).not.toContain("← Back");
    expect(modal).not.toContain("✕");
  });
});

describe("icon layout contract", () => {
  const css = readSource("src/App.css");

  it("defines theme-safe icon sizing and flex shrink rules", () => {
    expect(css).toContain("--icon-size");
    expect(css).toContain("--icon-gap");
    expect(css).toContain(".search-field");
    expect(css).toContain(".btn .icon");
    expect(css).toContain("min-width: 0");
    expect(css).toContain(".status-icon.clean");
    expect(css).toContain(".status-icon.modified");
    expect(css).toContain(".status-icon.unknown");
    expect(css).toContain(".card-cover-icon");
    expect(css).toContain("@media (prefers-reduced-motion: reduce)");
  });

  it("keeps action labels and glyphs on one centered axis", () => {
    const icon = readSource("src/components/Icon.tsx");

    expect(icon).toContain('case "hammer":');
    expect(icon).toContain('transform="scale(0.0234375)"');
    expect(icon).toContain('M968.533333 810.666667l-388.266666-388.266667');
    expect(icon).toContain('fill="currentColor" stroke="none"');
    expect(css).toContain(".card-actions .btn > .icon");
    expect(css).toContain("line-height: var(--icon-size-sm)");
    expect(css).toContain("align-self: center");
  });
});
