import type { EditorProfile, DetectedEditor } from "../lib/api";
import { shouldShowCustomExecutable } from "../lib/settingsUi";
import { Icon } from "./Icon";

interface SettingsEditorPanelProps {
  profile: EditorProfile;
  argumentsText: string;
  detectedEditors: DetectedEditor[];
  loading: boolean;
  error: string | null;
  validationError: string | null;
  onSelectExecutable: (executable: string) => void;
  onArgumentsChange: (value: string) => void;
  onChooseOther: () => void;
  onRetry: () => void;
}

const SOURCE_LABELS: Record<DetectedEditor["source"], string> = {
  path: "Path",
  registry: "Registry",
  standard: "Standard",
};

export function SettingsEditorPanel({
  profile,
  argumentsText,
  detectedEditors,
  loading,
  error,
  validationError,
  onSelectExecutable,
  onArgumentsChange,
  onChooseOther,
  onRetry,
}: SettingsEditorPanelProps) {
  const showCustomExecutable = shouldShowCustomExecutable(
    profile.executable,
    detectedEditors.map((editor) => editor.executable),
  );

  return (
    <div className="settings-panel settings-editor-panel">
      <section className="settings-section" aria-labelledby="settings-editor-choice-title">
        <div className="settings-section-heading">
          <div>
            <h3 id="settings-editor-choice-title" className="settings-section-title">
              <Icon name="code" size={18} />
              Editor application
            </h3>
            <p className="settings-section-description">
              Pick an installed editor or choose an executable yourself.
            </p>
          </div>
          {loading && <span className="settings-loading-label">Detecting…</span>}
        </div>

        <div className="settings-editor-list" role="list" aria-label="Detected editors">
          <div role="listitem">
            <button
              type="button"
              className={`settings-editor-choice${profile.executable.trim() === "" ? " is-selected" : ""}`}
              aria-pressed={profile.executable.trim() === ""}
              onClick={() => onSelectExecutable("")}
            >
              <span className="settings-editor-choice-icon" aria-hidden="true">
                <Icon name="ban" size={18} />
              </span>
              <span className="settings-editor-choice-main">
                <strong>Not configured</strong>
                <span className="settings-editor-path">Open in editor stays unavailable.</span>
              </span>
              <span className="settings-editor-source">None</span>
            </button>
          </div>
          {showCustomExecutable && (
            <div role="listitem">
              <button
                type="button"
                className="settings-editor-choice is-selected"
                aria-pressed={profile.executable.trim() !== ""}
                onClick={() => onSelectExecutable(profile.executable)}
              >
                <span className="settings-editor-choice-icon" aria-hidden="true">
                  <Icon name="folder-open" size={18} />
                </span>
                <span className="settings-editor-choice-main">
                  <strong>Custom executable</strong>
                  <span className="settings-editor-path mono" title={profile.executable}>
                    {profile.executable}
                  </span>
                </span>
                <span className="settings-editor-source">Other</span>
              </button>
            </div>
          )}
          {detectedEditors.map((editor) => (
            <div key={`${editor.id}:${editor.executable}`} role="listitem">
              <button
                type="button"
                className={`settings-editor-choice${
                  profile.executable.toLowerCase() === editor.executable.toLowerCase()
                    ? " is-selected"
                    : ""
                }`}
                aria-pressed={profile.executable.toLowerCase() === editor.executable.toLowerCase()}
                onClick={() => onSelectExecutable(editor.executable)}
              >
                <span className="settings-editor-choice-icon" aria-hidden="true">
                  <Icon name="code" size={18} />
                </span>
                <span className="settings-editor-choice-main">
                  <strong>{editor.name}</strong>
                  <span className="settings-editor-path mono" title={editor.executable}>
                    {editor.executable}
                  </span>
                </span>
                <span className="settings-editor-source">{SOURCE_LABELS[editor.source]}</span>
              </button>
            </div>
          ))}
        </div>

        <div className="settings-editor-actions">
          <button type="button" className="btn small" onClick={onChooseOther}>
            <Icon name="folder-open" size={14} />
            <span>Other…</span>
          </button>
          <span className="settings-hint">Only .exe files can be selected here.</span>
        </div>

        {error && (
          <div className="settings-inline-alert" role="alert">
            <div className="inline-alert-copy">
              <Icon name="alert-triangle" size={17} />
              <span>{error}</span>
            </div>
            <button type="button" className="link-btn" onClick={onRetry}>
              <Icon name="refresh" size={14} />
              Try again
            </button>
          </div>
        )}
        {!loading && !error && detectedEditors.length === 0 && (
          <p className="settings-hint">No supported editor was found. Use Other… to select one.</p>
        )}
      </section>

      <section className="settings-section" aria-labelledby="settings-editor-arguments-title">
        <div className="settings-section-heading">
          <div>
            <h3 id="settings-editor-arguments-title" className="settings-section-title">
              <Icon name="terminal" size={18} />
              Launch arguments
            </h3>
            <p className="settings-section-description">
              Enter one argument per line. The folder you open is passed through the one {"{path}"} token.
            </p>
          </div>
        </div>
        <label className="settings-field-label" htmlFor="settings-editor-arguments">
          <Icon name="align-left" size={15} />
          <span>Arguments</span>
        </label>
        <textarea
          id="settings-editor-arguments"
          className="settings-editor-arguments"
          rows={4}
          value={argumentsText}
          aria-invalid={validationError !== null}
          aria-describedby={validationError ? "settings-editor-validation-error" : "settings-editor-path-help"}
          onChange={(event) => onArgumentsChange(event.target.value)}
          spellCheck={false}
        />
        <p id="settings-editor-path-help" className="settings-hint">
          {"{path}"} is replaced with the project folder, so the editor opens that folder as its workspace.
        </p>
        {validationError && (
          <p id="settings-editor-validation-error" className="settings-error settings-inline-message" role="alert">
            <Icon name="alert-triangle" size={15} />
            <span>{validationError}</span>
          </p>
        )}
      </section>
    </div>
  );
}
