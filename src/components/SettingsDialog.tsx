// 设置 Dialog（D1 主题 + D6 编辑器）：
// 主题单选（跟随系统 / 亮 / 暗）与编辑器命令（预设 datalist + 自由输入）；
// 保存后经 update_settings 持久化到 settings.json，App 据此应用 data-theme。

import { useEffect, useState } from "react";
import { Modal } from "./Modal";
import { updateSettings, type AppSettings } from "../lib/api";
import { EDITOR_PRESETS, THEMES, isValidTheme, type Theme } from "../lib/theme";

export function SettingsDialog({
  settings,
  onClose,
  onSaved,
}: {
  settings: AppSettings | null;
  onClose: () => void;
  onSaved: (settings: AppSettings) => void;
}) {
  const [theme, setTheme] = useState<Theme>("system");
  const [editorCommand, setEditorCommand] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // settings 异步到达时初始化表单。
  useEffect(() => {
    if (!settings) {
      return;
    }
    setTheme(isValidTheme(settings.theme) ? settings.theme : "system");
    setEditorCommand(settings.editorCommand);
  }, [settings]);

  async function save() {
    setError(null);
    setBusy(true);
    try {
      const saved = await updateSettings({
        theme,
        editorCommand: editorCommand.trim(),
      });
      onSaved(saved);
      onClose();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  if (!settings) {
    return (
      <Modal
        title="Settings"
        onClose={onClose}
        footer={
          <button className="btn" onClick={onClose}>
            Close
          </button>
        }
      >
        <p className="muted">Loading settings…</p>
      </Modal>
    );
  }

  return (
    <Modal
      title="Settings"
      onClose={onClose}
      footer={
        <>
          <button className="btn" onClick={onClose}>
            Cancel
          </button>
          <button className="btn btn-primary" onClick={save} disabled={busy}>
            Save changes
          </button>
        </>
      }
    >
      <div className="settings-rows">
        <div>
          <h3 className="settings-section-title">Theme</h3>
          <div className="theme-options" role="radiogroup" aria-label="Theme">
            {THEMES.map((t) => (
              <label
                key={t.value}
                className="theme-option"
                role="radio"
                aria-checked={theme === t.value}
              >
                <input
                  type="radio"
                  name="theme"
                  value={t.value}
                  checked={theme === t.value}
                  onChange={() => setTheme(t.value)}
                />
                <span className={`theme-swatch ${t.value}`} aria-hidden="true" />
                <span className="theme-label">{t.label}</span>
              </label>
            ))}
          </div>
        </div>

        <div className="settings-field">
          <label htmlFor="editor-command">Editor command</label>
          <input
            id="editor-command"
            type="text"
            list="editor-presets"
            placeholder="code, code-insiders, cursor, or a path…"
            value={editorCommand}
            onChange={(e) => setEditorCommand(e.target.value)}
            autoComplete="off"
          />
          <datalist id="editor-presets">
            {EDITOR_PRESETS.map((p) => (
              <option key={p.command} value={p.command}>
                {p.label}
              </option>
            ))}
          </datalist>
          <p className="settings-hint">
            {editorCommand.trim()
              ? `Used by “Open in editor”. Running: ${editorCommand.trim()}`
              : "Not configured — “Open in editor” will report Editor not configured."}
          </p>
        </div>

        {error && <p className="settings-error">{error}</p>}
      </div>
    </Modal>
  );
}