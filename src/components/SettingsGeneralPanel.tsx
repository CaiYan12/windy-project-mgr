import type { AppInfo } from "../lib/api";

interface SettingsGeneralPanelProps {
  appInfo: AppInfo | null;
  loading: boolean;
  error: string | null;
  feedback: { kind: "success" | "error"; message: string } | null;
  resetBusy: boolean;
  onRetry: () => void;
  onReset: () => void;
}

export function SettingsGeneralPanel({
  appInfo,
  loading,
  error,
  feedback,
  resetBusy,
  onRetry,
  onReset,
}: SettingsGeneralPanelProps) {
  return (
    <div className="settings-panel settings-general-panel">
      <section className="settings-section" aria-labelledby="settings-app-info-title">
        <div className="settings-section-heading">
          <div>
            <h3 id="settings-app-info-title" className="settings-section-title">
              Application info
            </h3>
            <p className="settings-section-description">
              Windy runs locally and keeps its data beside the application.
            </p>
          </div>
          {loading && <span className="settings-loading-label">Reading…</span>}
        </div>
        {appInfo ? (
          <dl className="settings-info-list">
            <div>
              <dt>Version</dt>
              <dd className="mono">{appInfo.version}</dd>
            </div>
            <div>
              <dt>Data directory</dt>
              <dd className="mono settings-data-path" title={appInfo.dataDir}>
                {appInfo.dataDir}
              </dd>
            </div>
          </dl>
        ) : error ? (
          <div className="settings-inline-alert" role="alert">
            <span>{error}</span>
            <button type="button" className="link-btn" onClick={onRetry}>
              Try again
            </button>
          </div>
        ) : (
          <p className="settings-hint">Loading application information…</p>
        )}
      </section>

      <section className="settings-section settings-danger-section" aria-labelledby="settings-reset-title">
        <div>
          <h3 id="settings-reset-title" className="settings-section-title">
            Reset settings
          </h3>
          <p className="settings-section-description">
            Restore color, accent, and editor settings to the Rust defaults. Project records are not changed.
          </p>
        </div>
        <button type="button" className="btn btn-outline-danger" onClick={onReset} disabled={resetBusy}>
          {resetBusy ? "Resetting…" : "Reset settings"}
        </button>
        {feedback && (
          <p
            className={feedback.kind === "success" ? "settings-success" : "settings-error"}
            role={feedback.kind === "success" ? "status" : "alert"}
          >
            {feedback.message}
          </p>
        )}
      </section>
    </div>
  );
}
