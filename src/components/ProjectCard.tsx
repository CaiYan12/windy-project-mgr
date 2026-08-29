// Project Card（原始文档第 11 节）：信息不足优雅降级；扫描中显示骨架；
// 单项采集失败不阻断卡片显示。More 菜单：在编辑器中打开（D6 入口）/ Edit / Delete。

import { useState } from "react";
import type { Project, ScanState } from "../types/project";
import { activityText, gitLine, gitSyncLine } from "../lib/cards";
import { Icon, type IconName } from "./Icon";

const GIT_STATUS_ICONS: Record<"clean" | "modified" | "unknown", IconName> = {
  clean: "check-circle",
  modified: "alert-triangle",
  unknown: "help-circle",
};

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
  onSelect,
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
  onSelect: () => void;
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
    <article className="card clickable" onClick={onSelect}>
      <header className="card-header">
          <div className="card-title">
          <h3>
            <button
              className="card-title-link"
              onClick={(e) => {
                e.stopPropagation();
                onSelect();
              }}
            >
              {project.name}
            </button>
          </h3>
          <span className="type-chip">
            <Icon name="overview" size={13} />
            <span>{meta?.projectType ?? "Unknown project type"}</span>
          </span>
        </div>
        <div className="card-menu-wrap">
          <button
            className="icon-btn"
            aria-label="More"
            title="More actions"
            aria-expanded={menuOpen}
            onClick={(e) => {
              e.stopPropagation();
              setMenuOpen((v) => !v);
            }}
          >
            <Icon name="more-horizontal" size={18} />
          </button>
          {menuOpen && (
            <>
              <div
                className="menu-overlay"
                onClick={(e) => {
                  e.stopPropagation();
                  setMenuOpen(false);
                }}
              />
              <div className="menu" role="menu">
                <button
                  role="menuitem"
                  onClick={(e) => {
                    e.stopPropagation();
                    setMenuOpen(false);
                    onOpenInEditor();
                  }}
                >
                  <Icon name="code" size={16} />
                  <span>Open in editor</span>
                </button>
                <button
                  role="menuitem"
                  onClick={(e) => {
                    e.stopPropagation();
                    setMenuOpen(false);
                    onEdit();
                  }}
                >
                  <Icon name="pencil" size={16} />
                  <span>Edit</span>
                </button>
                <button
                  role="menuitem"
                  className="menu-danger"
                  onClick={(e) => {
                    e.stopPropagation();
                    setMenuOpen(false);
                    onDelete();
                  }}
                >
                  <Icon name="trash" size={16} />
                  <span>Delete</span>
                </button>
              </div>
            </>
          )}
        </div>
      </header>

      <p className="card-path">
        <Icon name="folder-open" size={15} />
        <span>{project.path}</span>
      </p>
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
          <Icon name="alert-triangle" size={16} />
          <span>Scan failed: {scan.message}</span>{" "}
          <button
            className="link-btn"
            onClick={(e) => {
              e.stopPropagation();
              onRescan();
            }}
            >
            <Icon name="refresh" size={14} />
            <span>Retry</span>
          </button>
        </p>
      ) : (
        meta && (
          <>
            <p className="git-line">
            {meta.git === null ? (
                <>
                  <Icon name="help-circle" size={16} />
                  <span>No Git repository</span>
                </>
              ) : (
                <>
                  <Icon
                    name={GIT_STATUS_ICONS[meta.git.status]}
                    size={16}
                    className={`status-icon ${meta.git.status}`}
                  />
                  <span className="mono">{gitLine(meta.git)}</span>
                  {syncLine && <span className="mono muted"> · {syncLine}</span>}
                </>
              )}
            </p>
            <p className="activity-line">
              <Icon name="activity" size={15} />
              <span>{activityText(meta.activity, now)}</span>
            </p>
          </>
        )
      )}

      <footer className="card-actions">
        <button
          className="btn small"
          onClick={(e) => {
            e.stopPropagation();
            onOpen();
          }}
        >
          <Icon name="folder-open" size={16} />
          <span>Open</span>
        </button>
        <button
          className="btn small"
          onClick={(e) => {
            e.stopPropagation();
            onRun();
          }}
          disabled={!project.runCommand}
          title={project.runCommand ? undefined : "Run command not configured"}
        >
          <Icon name="play" size={16} />
          <span>Run</span>
        </button>
        <button
          className="btn small"
          onClick={(e) => {
            e.stopPropagation();
            onBuild();
          }}
          disabled={!project.buildCommand}
          title={project.buildCommand ? undefined : "Build command not configured"}
        >
          <Icon name="hammer" size={16} />
          <span>Build</span>
        </button>
      </footer>
    </article>
  );
}
