// Project Card（原始文档第 11 节）：信息不足优雅降级；扫描中显示骨架；
// 单项采集失败不阻断卡片显示。More 菜单：在编辑器中打开（D6 入口）/ Edit / Delete。

import { useState } from "react";
import type { Project, ScanState } from "../types/project";
import { activityText, gitLine, gitSyncLine } from "../lib/cards";

export function ProjectCardSkeleton() {
  return (
    <div className="card scanning" aria-busy="true">
      <div className="skel skel-title" />
      <div className="skel skel-line" />
      <div className="skel skel-line short" />
      <div className="skel skel-footer" />
    </div>
  );
}

export function ProjectCard({
  project,
  scan,
  onOpen,
  onRun,
  onBuild,
  onOpenInEditor,
  onEdit,
  onDelete,
  onRescan,
}: {
  project: Project;
  scan?: ScanState;
  onOpen: () => void;
  onRun: () => void;
  onBuild: () => void;
  onOpenInEditor: () => void;
  onEdit: () => void;
  onDelete: () => void;
  onRescan: () => void;
}) {
  const [menuOpen, setMenuOpen] = useState(false);

  if (!scan || scan.status === "loading") {
    return <ProjectCardSkeleton />;
  }

  const now = Date.now();
  const meta = scan.status === "ok" ? scan.data : null;
  const syncLine = meta?.git ? gitSyncLine(meta.git) : null;

  return (
    <article className="card">
      <header className="card-header">
        <div className="card-title">
          <h3>{project.name}</h3>
          <span className="type-chip">{meta?.projectType ?? "Unknown project type"}</span>
        </div>
        <div className="card-menu-wrap">
          <button
            className="icon-btn"
            aria-label="More"
            aria-expanded={menuOpen}
            onClick={() => setMenuOpen((v) => !v)}
          >
            ⋯
          </button>
          {menuOpen && (
            <>
              <div className="menu-overlay" onClick={() => setMenuOpen(false)} />
              <div className="menu" role="menu">
                <button
                  role="menuitem"
                  onClick={() => {
                    setMenuOpen(false);
                    onOpenInEditor();
                  }}
                >
                  Open in editor
                </button>
                <button
                  role="menuitem"
                  onClick={() => {
                    setMenuOpen(false);
                    onEdit();
                  }}
                >
                  Edit
                </button>
                <button
                  role="menuitem"
                  className="menu-danger"
                  onClick={() => {
                    setMenuOpen(false);
                    onDelete();
                  }}
                >
                  Delete
                </button>
              </div>
            </>
          )}
        </div>
      </header>

      <p className="card-path">{project.path}</p>
      <p className="card-desc">{project.description || "No description"}</p>

      {meta && meta.techStack.length > 0 && (
        <div className="tech-row">
          {meta.techStack.map((t) => (
            <span key={t} className="tech-chip">
              {t}
            </span>
          ))}
        </div>
      )}

      {scan.status === "error" ? (
        <p className="card-error">
          Scan failed: {scan.message}{" "}
          <button className="link-btn" onClick={onRescan}>
            Retry
          </button>
        </p>
      ) : (
        meta && (
          <>
            <p className="git-line">
              {meta.git === null ? (
                "No Git repository"
              ) : (
                <>
                  <span className={`status-dot ${meta.git.status}`} />
                  <span className="mono">{gitLine(meta.git)}</span>
                  {syncLine && <span className="mono muted"> · {syncLine}</span>}
                </>
              )}
            </p>
            <p className="activity-line">{activityText(meta.activity, now)}</p>
          </>
        )
      )}

      <footer className="card-actions">
        <button className="btn small" onClick={onOpen}>
          Open
        </button>
        <button
          className="btn small"
          onClick={onRun}
          disabled={!project.runCommand}
          title={project.runCommand ? undefined : "Run command not configured"}
        >
          Run
        </button>
        <button
          className="btn small"
          onClick={onBuild}
          disabled={!project.buildCommand}
          title={project.buildCommand ? undefined : "Build command not configured"}
        >
          Build
        </button>
      </footer>
    </article>
  );
}
