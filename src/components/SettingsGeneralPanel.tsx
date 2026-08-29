import type { AppInfo } from "../lib/api";
import { Icon } from "./Icon";

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
              <Icon name="info" size={18} />
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
            <dt>
              <Icon name="info" size={15} />
              <span>Version</span>
            </dt>
              <dd className="mono">{appInfo.version}</dd>
            </div>
          <div>
            <dt>
              <Icon name="folder-open" size={15} />
              <span>Data directory</span>
            </dt>
              <dd className="mono settings-data-path" title={appInfo.dataDir}>
                {appInfo.dataDir}
              </dd>
            </div>
          </dl>
        ) : error ? (
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
        ) : (
          <p className="settings-hint">Loading application information…</p>
        )}
      </section>

      <section className="settings-section settings-danger-section" aria-labelledby="settings-reset-title">
        <div>
          <h3 id="settings-reset-title" className="settings-section-title">
            <Icon name="rotate-ccw" size={18} />
            Reset settings
          </h3>
          <p className="settings-section-description">
            Restore color, accent, and editor settings to the Rust defaults. Project records are not changed.
          </p>
        </div>
        <button type="button" className="btn btn-outline-danger" onClick={onReset} disabled={resetBusy}>
          <Icon name="rotate-ccw" size={15} />
          <span>{resetBusy ? "Resetting…" : "Reset settings"}</span>
        </button>
        {feedback && (
          <p
            className={feedback.kind === "success" ? "settings-success" : "settings-error"}
            role={feedback.kind === "success" ? "status" : "alert"}
          >
            <Icon name={feedback.kind === "success" ? "check-circle" : "alert-triangle"} size={15} />
            <span>
            {feedback.message}
            </span>
          </p>
        )}
      </section>
    </div>
  );
}
