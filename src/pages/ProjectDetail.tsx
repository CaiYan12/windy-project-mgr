// Project Detail（Phase 9，原始文档第 12 节）：只读展示选中项目的全部扫描信息。
// D2：扫描结果仅存前端内存——本页不发起扫描、不持久化，只消费传入的 scan 状态；
// 未扫描 / 扫描中显示骨架，扫描失败显示错误 + Retry。
// D6：Open in editor 等操作按钮经由 props 回调接入，本页不直接调用 IPC。

import type { Project, ScanState } from "../types/project";
import { activityText, gitLine, gitSyncLine, relativeTime, shortHash } from "../lib/cards";

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
          ← Back
        </button>
        <div className="detail-title">
          <h2>{project.name}</h2>
          <span className="type-chip">
            {(scan?.status === "ok" ? scan.data.projectType : null) ?? "Unknown project type"}
          </span>
        </div>
        <p className="mono muted detail-path">{project.path}</p>
      </header>

      {!scan || scan.status === "loading" ? (
        <div className="detail-skeleton" aria-busy="true">
          <div className="skel skel-title" />
          <div className="skel skel-line" />
          <div className="skel skel-line short" />
          <div className="skel skel-footer" />
        </div>
      ) : scan.status === "error" ? (
        <div className="detail-section detail-error">
          <p>Scan failed: {scan.message}</p>
          <button className="btn small" onClick={onRefresh}>
            Retry
          </button>
        </div>
      ) : (
        (() => {
          const meta = scan.data;
          const syncLine = meta.git ? gitSyncLine(meta.git) : null;
          return (
            <div className="detail-grid">
              <section className="detail-section">
                <h3 className="detail-section-title">Overview</h3>
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
                    <dt>Created</dt>
                    <dd>{project.createdAt}</dd>
                  </div>
                  <div className="detail-kv-row">
                    <dt>Run command</dt>
                    <dd>
                      {project.runCommand ? (
                        <code className="mono">{project.runCommand}</code>
                      ) : (
                        <span className="muted">Run command not configured</span>
                      )}
                    </dd>
                  </div>
                  <div className="detail-kv-row">
                    <dt>Build command</dt>
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
                <h3 className="detail-section-title">Technology</h3>
                <div className="tech-row">
                  <span className="type-chip">{meta.projectType ?? "Unknown project type"}</span>
                  {meta.techStack.map((t) => (
                    <span key={t} className="tech-chip">
                      {t}
                    </span>
                  ))}
                </div>
                {meta.techStack.length === 0 && (
                  <p className="muted detail-empty-inline">No tech stack detected</p>
                )}
              </section>

              <section className="detail-section">
                <h3 className="detail-section-title">Git</h3>
                {meta.git === null ? (
                  <p className="muted">No Git repository</p>
                ) : (
                  <>
                    <p className="git-line">
                      <span className={`status-dot ${meta.git.status}`} />
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
                      <p className="muted">No commits yet</p>
                    )}
                  </>
                )}
              </section>

              <section className="detail-section">
                <h3 className="detail-section-title">Recent Commits</h3>
                {meta.git === null ? (
                  <p className="muted">No Git repository</p>
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
                  <p className="muted">No commits yet</p>
                )}
              </section>

              <section className="detail-section">
                <h3 className="detail-section-title">Activity</h3>
                <p className="detail-activity">{activityText(meta.activity, now)}</p>
                <p className="mono muted">Scanned {relativeTime(meta.activity.lastScannedAt, now)}</p>
              </section>

              <section className="detail-section">
                <h3 className="detail-section-title">Actions</h3>
                <div className="detail-actions">
                  <button className="btn" onClick={onOpen}>
                    Open
                  </button>
                  <button
                    className="btn"
                    onClick={onRun}
                    disabled={!project.runCommand}
                    title={project.runCommand ? undefined : "Run command not configured"}
                  >
                    Run
                  </button>
                  <button
                    className="btn"
                    onClick={onBuild}
                    disabled={!project.buildCommand}
                    title={project.buildCommand ? undefined : "Build command not configured"}
                  >
                    Build
                  </button>
                  <button className="btn" onClick={onOpenInEditor}>
                    Open in editor
                  </button>
                  <button className="btn" onClick={onRefresh}>
                    Refresh
                  </button>
                  <button className="btn" onClick={onEdit}>
                    Edit
                  </button>
                  <button className="btn btn-danger" onClick={onDelete}>
                    Delete
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
