// Add / Edit Dialog 共享的字段表单（受控）。

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
  return (
    <div className="form-grid">
      <label className="field">
        <span className="field-label">Path</span>
        <input
          value={state.path}
          onChange={(e) => onChange({ path: e.target.value })}
          placeholder="d:\Dev\my-project"
          autoFocus
        />
      </label>

      <label className="field">
        <span className="field-label">
          Name{nameAutoHint ? " (auto-filled from path)" : ""}
        </span>
        <input
          value={state.name}
          onChange={(e) => onChange({ name: e.target.value })}
        />
      </label>

      <label className="field">
        <span className="field-label">Description</span>
        <input
          value={state.description}
          onChange={(e) => onChange({ description: e.target.value })}
        />
      </label>

      <label className="field">
        <span className="field-label">Tags (comma-separated)</span>
        <input
          value={state.tagsText}
          onChange={(e) => onChange({ tagsText: e.target.value })}
          placeholder="web, tool"
        />
      </label>

      {showCommands && (
        <>
          <label className="field">
            <span className="field-label">Run command</span>
            <input
              value={state.runCommand}
              onChange={(e) => onChange({ runCommand: e.target.value })}
              placeholder="pnpm dev"
            />
          </label>

          <label className="field">
            <span className="field-label">Build command</span>
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
