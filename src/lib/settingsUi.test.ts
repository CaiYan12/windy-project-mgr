import { describe, expect, it } from "vitest";
// @ts-expect-error The application intentionally does not ship Node typings; this is a Vitest-only static contract test.
import { readFileSync } from "node:fs";
// @ts-expect-error The application intentionally does not ship Node typings; this is a Vitest-only static contract test.
import { resolve } from "node:path";
declare const process: { cwd: () => string };
import {
  createRequestGeneration,
  isSettingsCloseBlocked,
  nextRadioIndex,
  shouldShowCustomExecutable,
} from "./settingsUi";

describe("settings UI pure interaction guards", () => {
  it("blocks every settings close path while save or reset is pending", () => {
    expect(isSettingsCloseBlocked(false, false)).toBe(false);
    expect(isSettingsCloseBlocked(true, false)).toBe(true);
    expect(isSettingsCloseBlocked(false, true)).toBe(true);
  });

  it("moves an accent radio selection with Arrow, Home, and End", () => {
    expect(nextRadioIndex(0, "ArrowRight", 8)).toBe(1);
    expect(nextRadioIndex(0, "ArrowLeft", 8)).toBe(7);
    expect(nextRadioIndex(3, "Home", 8)).toBe(0);
    expect(nextRadioIndex(3, "End", 8)).toBe(7);
    expect(nextRadioIndex(3, "Enter", 8)).toBeNull();
  });

  it("ignores an earlier asynchronous response after a newer request starts", () => {
    const generation = createRequestGeneration();
    const first = generation.next();
    const second = generation.next();

    expect(generation.isCurrent(first)).toBe(false);
    expect(generation.isCurrent(second)).toBe(true);
  });

  it("invalidates the current asynchronous response when its owner unmounts", () => {
    const generation = createRequestGeneration();
    const request = generation.next();

    generation.invalidate();

    expect(generation.isCurrent(request)).toBe(false);
  });

  it("shows only a non-empty executable that is absent from detected editors", () => {
    expect(shouldShowCustomExecutable("", ["C:\\Editors\\Code.exe"])).toBe(false);
    expect(shouldShowCustomExecutable("C:\\Editors\\Code.exe", ["C:\\Editors\\Code.exe"])).toBe(false);
    expect(shouldShowCustomExecutable("c:\\editors\\code.exe", ["C:\\Editors\\Code.exe"])).toBe(false);
    expect(shouldShowCustomExecutable("C:\\Tools\\custom.exe", ["C:\\Editors\\Code.exe"])).toBe(true);
  });
});

describe("settings UI static accessibility and layout contracts", () => {
  const dialog = readSource("src/components/SettingsDialog.tsx");
  const appearance = readSource("src/components/SettingsAppearancePanel.tsx");
  const editor = readSource("src/components/SettingsEditorPanel.tsx");
  const modal = readSource("src/components/Modal.tsx");
  const app = readSource("src/App.tsx");
  const css = readSource("src/App.css");

  it("keeps every tab control connected to a DOM panel", () => {
    expect(dialog).toContain('TABS.map((tab) => (');
    expect(dialog).toContain('id={`settings-panel-${tab.id}`}');
    expect(dialog).toContain('hidden={activeTab !== tab.id}');
  });

  it("uses radio semantics and keyboard handling for accent choices", () => {
    expect(appearance).toContain('role="radio"');
    expect(appearance).toContain("aria-checked={selected}");
    expect(appearance).toContain("nextRadioIndex");
    expect(appearance).toContain("onAccentKeyDown");
    expect(appearance).not.toContain("aria-pressed");
  });

  it("keeps dialog focus inside and protects pending close paths", () => {
    expect(modal).toContain("closeDisabled");
    expect(modal).toContain("aria-modal=\"true\"");
    expect(modal).toContain("document.activeElement");
    expect(modal).toContain('e.key !== "Tab"');
    expect(modal).toContain('!element.closest("[hidden]")');
    expect(dialog).toContain("closeDisabled={isSettingsBusy}");
    expect(dialog).toContain("if (isSettingsBusy)");
  });

  it("guards async editor/general responses and keeps editor list semantics valid", () => {
    expect(dialog).toContain("createRequestGeneration");
    expect(dialog).toContain("editorRequestGeneration");
    expect(dialog).toContain("appInfoRequestGeneration");
    expect(editor).toContain('role="listitem"');
  });

  it("shows an unlisted custom executable as a visible selected editor choice", () => {
    const customChoiceStart = editor.indexOf("{showCustomExecutable && (");
    const customChoiceEnd = editor.indexOf("{detectedEditors.map", customChoiceStart);
    expect(customChoiceStart).toBeGreaterThanOrEqual(0);
    expect(customChoiceEnd).toBeGreaterThan(customChoiceStart);

    const customChoice = editor.slice(customChoiceStart, customChoiceEnd);
    expect(customChoice).toContain('className="settings-editor-choice is-selected"');
    expect(customChoice).toContain('aria-pressed={profile.executable.trim() !== ""}');
    expect(customChoice).toContain("<strong>Custom executable</strong>");
    expect(customChoice).toContain("{profile.executable}");
    expect(customChoice).toContain('<span className="settings-editor-source">Other</span>');

  });

  it("synchronizes dialog Windows accent results and listens for system changes", () => {
    expect(dialog).toContain("onWindowsAccentResult");
    expect(app).toContain("PREFERS_DARK_QUERY");
    expect(app).toContain("addEventListener(\"change\"");
    expect(app).toContain("onWindowsAccentResult");
  });

  it("invalidates every dialog request generation during unmount cleanup", () => {
    expect(dialog).toContain("return () => {");
    expect(dialog).toContain("windowsAccentRequestGeneration.current.invalidate()");
    expect(dialog).toContain("editorRequestGeneration.current.invalidate()");
    expect(dialog).toContain("appInfoRequestGeneration.current.invalidate()");
  });

  it("gives the inline reset alertdialog a description and its own focus handoff", () => {
    expect(dialog).toContain('role="alertdialog"');
    expect(dialog).toContain('aria-describedby="settings-reset-confirm-description"');
    expect(dialog).toContain('id="settings-reset-confirm-description"');
    expect(dialog).toContain("resetConfirmTriggerRef");
    expect(dialog).toContain("resetConfirmFirstActionRef");
    expect(dialog).toContain("resetConfirmFirstActionRef.current?.focus()");
  });

  it("allows inline alerts to shrink and break long paths on narrow screens", () => {
    expect(css).toContain(".settings-inline-alert > span");
    expect(css).toContain("min-width: 0");
    expect(css).toContain("overflow-wrap: anywhere");
    expect(css).not.toContain(".theme-options");
    expect(css).not.toContain(".theme-option");
    expect(css).not.toContain(".settings-rows");
    expect(css.match(/\.settings-section-title\s*\{/g)).toHaveLength(1);
  });
});
const readSource = (relativePath: string) =>
  readFileSync(resolve(process.cwd(), relativePath), "utf8");
