// Add / Edit Dialog 共享的字段表单（受控）。
// Path 字段带「Browse…」按钮：触发 Windows 原生文件夹选择对话框
//（复用 @tauri-apps/plugin-dialog，Step 2 文件选择同源；D10 路径选填体验）。
// 最近一次浏览目录记入 localStorage，下次打开定位到该处。

import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { Icon } from "./Icon";

/** localStorage 键：最近一次浏览的目录（供 Browse… 默认定位）。 */
const LAST_BROWSE_KEY = "windy:last-browse-dir";

export interface ProjectFormState {
  name: string;
  path: string;
  description: string;
  tagsText: string;
  runCommand: string;
  buildCommand: string;
}

export function parseTags(text: string): string[] {
  return text
    .split(",")
    .map((t) => t.trim())
    .filter((t) => t.length > 0);
}

export function ProjectFields({
  state,
  onChange,
  showCommands,
  nameAutoHint,
}: {
  state: ProjectFormState;
  onChange: (patch: Partial<ProjectFormState>) => void;
  showCommands: boolean;
  nameAutoHint?: boolean;
}) {
  // 选择目录：对话框默认定位到上次浏览目录；选中后记住该目录。
  // 取消返回 null 时不改动；失败等同取消（仍可手动输入）。
  async function browsePath() {
    try {
      const dir = await openDialog({
        directory: true,
        defaultPath: localStorage.getItem(LAST_BROWSE_KEY) ?? undefined,
      });
      if (typeof dir === "string") {
        localStorage.setItem(LAST_BROWSE_KEY, dir);
        onChange({ path: dir });
      }
    } catch {
      /* 选择器不可用或取消：保持原输入 */
    }
  }

  return (
    <div className="form-grid">
      <label className="field">
        <span className="field-label">
          <Icon name="folder-open" size={15} />
          <span>Path</span>
        </span>
        <div className="field-row">
          <input
            value={state.path}
            onChange={(e) => onChange({ path: e.target.value })}
            placeholder="d:\Dev\my-project"
            autoFocus
          />
          <button type="button" className="btn small" onClick={browsePath}>
            <Icon name="folder-open" size={14} />
            <span>Browse…</span>
          </button>
        </div>
      </label>

      <label className="field">
        <span className="field-label">
          <Icon name="text" size={15} />
          <span>
          Name{nameAutoHint ? " (auto-filled from path)" : ""}
          </span>
        </span>
        <input
          value={state.name}
          onChange={(e) => onChange({ name: e.target.value })}
        />
      </label>

      <label className="field">
        <span className="field-label">
          <Icon name="align-left" size={15} />
          <span>Description</span>
        </span>
        <input
          value={state.description}
          onChange={(e) => onChange({ description: e.target.value })}
        />
      </label>

      <label className="field">
        <span className="field-label">
          <Icon name="tag" size={15} />
          <span>Tags (comma-separated)</span>
        </span>
        <input
          value={state.tagsText}
          onChange={(e) => onChange({ tagsText: e.target.value })}
          placeholder="web, tool"
        />
      </label>

      {showCommands && (
        <>
          <label className="field">
            <span className="field-label">
              <Icon name="terminal" size={15} />
              <span>Run command</span>
            </span>
            <input
              value={state.runCommand}
              onChange={(e) => onChange({ runCommand: e.target.value })}
              placeholder="pnpm dev"
            />
          </label>

          <label className="field">
            <span className="field-label">
              <Icon name="hammer" size={15} />
              <span>Build command</span>
            </span>
            <input
              value={state.buildCommand}
              onChange={(e) => onChange({ buildCommand: e.target.value })}
              placeholder="pnpm build"
            />
          </label>
        </>
      )}
    </div>
  );
}
