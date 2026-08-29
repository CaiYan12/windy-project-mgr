// Project Detail（Phase 9，原始文档第 12 节）：只读展示选中项目的全部扫描信息。
// D2：扫描结果仅存前端内存——本页不发起扫描、不持久化，只消费传入的 scan 状态；
// 未扫描 / 扫描中显示骨架，扫描失败显示错误 + Retry。
// D6：Open in editor 等操作按钮经由 props 回调接入，本页不直接调用 IPC。

import type { Project, ScanState } from "../types/project";
import { activityText, formatDateTime, gitLine, gitSyncLine, relativeTime, shortHash } from "../lib/cards";
import { Icon, type IconName } from "../components/Icon";

const GIT_STATUS_ICONS: Record<"clean" | "modified" | "unknown", IconName> = {
  clean: "check-circle",
  modified: "alert-triangle",
  unknown: "help-circle",
};

export function ProjectDetail({
  project,
  scan,
  onBack,
  onRefresh,
  onOpen,
  onRun,
  onBuild,
  onOpenInEditor,
  onEdit,
  onDelete,
}: {
  project: Project;
  scan?: ScanState;
  onBack: () => void;
  onRefresh: () => void;
  onOpen: () => void;
  onRun: () => void;
  onBuild: () => void;
  onOpenInEditor: () => void;
  onEdit: () => void;
  onDelete: () => void;
}) {
  const now = Date.now();

  return (
    <div className="detail">
      <header className="detail-header">
        <button className="btn small" onClick={onBack}>
          <Icon name="arrow-left" size={15} />
          <span>Back</span>
        </button>
        <div className="detail-title">
          <h2>{project.name}</h2>
          <span className="type-chip">
            <Icon name="overview" size={13} />
            {(scan?.status === "ok" ? scan.data.projectType : null) ?? "Unknown project type"}
          </span>
        </div>
        <p className="mono muted detail-path">
          <Icon name="folder-open" size={15} />
          <span>{project.path}</span>
        </p>
      </header>

      {!scan || scan.status === "loading" ? (
        <div className="detail-skeleton" aria-busy="true">
          <Icon name="refresh" size={20} className="detail-skeleton-icon" />
          <div className="skel skel-title" />
          <div className="skel skel-line" />
          <div className="skel skel-line short" />
          <div className="skel skel-footer" />
        </div>
      ) : scan.status === "error" ? (
        <div className="detail-section detail-error">
          <p className="detail-error-message">
            <Icon name="alert-triangle" size={18} />
            <span>Scan failed: {scan.message}</span>
          </p>
          <button className="btn small" onClick={onRefresh}>
            <Icon name="refresh" size={15} />
            <span>Retry</span>
          </button>
        </div>
      ) : (
        (() => {
          const meta = scan.data;
          const syncLine = meta.git ? gitSyncLine(meta.git) : null;
          return (
            <div className="detail-grid">
              <section className="detail-section">
                <h3 className="detail-section-title">
                  <Icon name="overview" size={16} />
                  <span>Overview</span>
                </h3>
                <p className="detail-desc">{project.description || "No description"}</p>
                {project.tags.length > 0 && (
                  <div className="tech-row">
                    {project.tags.map((t) => (
                      <span key={t} className="tech-chip">
                        {t}
                      </span>
                    ))}
                  </div>
                )}
                <dl className="detail-kv">
                  <div className="detail-kv-row">
                    <dt>
                      <Icon name="history" size={14} />
                      <span>Created</span>
                    </dt>
                    <dd>{formatDateTime(project.createdAt)}</dd>
                  </div>
                  <div className="detail-kv-row">
                    <dt>
                      <Icon name="play" size={14} />
                      <span>Run command</span>
                    </dt>
                    <dd>
                      {project.runCommand ? (
                        <code className="mono">{project.runCommand}</code>
                      ) : (
                        <span className="muted">Run command not configured</span>
                      )}
                    </dd>
                  </div>
                  <div className="detail-kv-row">
                    <dt>
                      <Icon name="hammer" size={14} />
                      <span>Build command</span>
                    </dt>
                    <dd>
                      {project.buildCommand ? (
                        <code className="mono">{project.buildCommand}</code>
                      ) : (
                        <span className="muted">Build command not configured</span>
                      )}
                    </dd>
                  </div>
                </dl>
              </section>

              <section className="detail-section">
                <h3 className="detail-section-title">
                  <Icon name="layers" size={16} />
                  <span>Technology</span>
                </h3>
                <div className="tech-row">
                  <span className="type-chip">
                    <Icon name="overview" size={13} />
                    <span>{meta.projectType ?? "Unknown project type"}</span>
                  </span>
                  {meta.techStack.map((t) => (
                    <span key={t} className="tech-chip">
                      {t}
                    </span>
                  ))}
                </div>
                {meta.techStack.length === 0 && (
                  <p className="muted detail-empty-inline">
                    <Icon name="layers" size={15} />
                    <span>No tech stack detected</span>
                  </p>
                )}
              </section>

              <section className="detail-section">
                <h3 className="detail-section-title">
                  <Icon name="git-branch" size={16} />
                  <span>Git</span>
                </h3>
                {meta.git === null ? (
                  <p className="muted detail-inline-message">
                    <Icon name="help-circle" size={16} />
                    <span>No Git repository</span>
                  </p>
                ) : (
                  <>
                    <p className="git-line">
                      <Icon
                        name={GIT_STATUS_ICONS[meta.git.status]}
                        size={16}
                        className={`status-icon ${meta.git.status}`}
                      />
                      <span className="mono">{gitLine(meta.git)}</span>
                      {syncLine && <span className="mono muted"> · {syncLine}</span>}
                    </p>
                    {meta.git.lastCommit ? (
                      <p className="detail-last-commit">
                        <span className="mono detail-hash">
                          {shortHash(meta.git.lastCommit.hash)}
                        </span>
                        <span>{meta.git.lastCommit.message}</span>
                        <span className="muted">
                          {meta.git.lastCommit.author} ·{" "}
                          {relativeTime(meta.git.lastCommit.date, now)}
                        </span>
                      </p>
                    ) : (
                      <p className="muted detail-inline-message">
                        <Icon name="history" size={16} />
                        <span>No commits yet</span>
                      </p>
                    )}
                  </>
                )}
              </section>

              <section className="detail-section">
                <h3 className="detail-section-title">
                  <Icon name="history" size={16} />
                  <span>Recent Commits</span>
                </h3>
                {meta.git === null ? (
                  <p className="muted detail-inline-message">
                    <Icon name="help-circle" size={16} />
                    <span>No Git repository</span>
                  </p>
                ) : meta.git.recentCommits.length > 0 ? (
                  <div className="commit-list">
                    {meta.git.recentCommits.map((c) => (
                      <div key={c.hash} className="commit-row">
                        <span className="mono detail-hash">{shortHash(c.hash)}</span>
                        <span className="commit-message">{c.message}</span>
                        <span className="muted">{c.author}</span>
                        <span className="muted">{relativeTime(c.date, now)}</span>
                      </div>
                    ))}
                  </div>
                ) : (
                  <p className="muted detail-inline-message">
                    <Icon name="history" size={16} />
                    <span>No commits yet</span>
                  </p>
                )}
              </section>

              <section className="detail-section">
                <h3 className="detail-section-title">
                  <Icon name="activity" size={16} />
                  <span>Activity</span>
                </h3>
                <p className="detail-activity">{activityText(meta.activity, now)}</p>
                <p className="mono muted">Scanned {relativeTime(meta.activity.lastScannedAt, now)}</p>
              </section>

              <section className="detail-section">
                <h3 className="detail-section-title">
                  <Icon name="zap" size={16} />
                  <span>Actions</span>
                </h3>
                <div className="detail-actions">
                  <button className="btn" onClick={onOpen}>
                    <Icon name="folder-open" size={16} />
                    <span>Open</span>
                  </button>
                  <button
                    className="btn"
                    onClick={onRun}
                    disabled={!project.runCommand}
                    title={project.runCommand ? undefined : "Run command not configured"}
                  >
                    <Icon name="play" size={16} />
                    <span>Run</span>
                  </button>
                  <button
                    className="btn"
                    onClick={onBuild}
                    disabled={!project.buildCommand}
                    title={project.buildCommand ? undefined : "Build command not configured"}
                  >
                    <Icon name="hammer" size={16} />
                    <span>Build</span>
                  </button>
                  <button className="btn" onClick={onOpenInEditor}>
                    <Icon name="code" size={16} />
                    <span>Open in editor</span>
                  </button>
                  <button className="btn" onClick={onRefresh}>
                    <Icon name="refresh" size={16} />
                    <span>Refresh</span>
                  </button>
                  <button className="btn" onClick={onEdit}>
                    <Icon name="pencil" size={16} />
                    <span>Edit</span>
                  </button>
                  <button className="btn btn-outline-danger" onClick={onDelete}>
                    <Icon name="trash" size={16} />
                    <span>Delete</span>
                  </button>
                </div>
              </section>
            </div>
          );
        })()
      )}
    </div>
  );
}
