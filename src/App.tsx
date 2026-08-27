// Dashboard（原始文档第 10 节：Sidebar + Toolbar + Card Grid）。
// D2：启动即渲染卡片骨架，并发扫描逐卡填充；结果仅存内存。

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Sidebar, type TagCount } from "./components/Sidebar";
import { ProjectCard } from "./components/ProjectCard";
import { ProjectDetail } from "./pages/ProjectDetail";
import { AddProjectDialog } from "./components/AddProjectDialog";
import { EditProjectDialog } from "./components/EditProjectDialog";
import { SettingsDialog } from "./components/SettingsDialog";
import { ConfirmDialog } from "./components/Modal";
import { filterProjects } from "./lib/search";
import {
  buildProject,
  deleteProject,
  getProjects,
  getSettings,
  getWindowsAccentColor,
  openInEditor,
  openProject,
  runProject,
  scanProject,
  type AppSettings,
} from "./lib/api";
import {
  applyTheme,
  captureThemeState,
  isValidHexColor,
  restoreThemeState,
  previewTheme,
  PREFERS_DARK_QUERY,
  prefersDarkColorScheme,
  type ThemeState,
} from "./lib/theme";
import { createRequestGeneration } from "./lib/settingsUi";
import type { Project, ScanState } from "./types/project";
import "./App.css";

type Dialog =
  | { kind: "add" }
  | { kind: "edit"; project: Project }
  | { kind: "delete"; project: Project }
  | { kind: "settings" }
  | null;

type SettingsPreview = {
  colorMode: AppSettings["colorMode"];
  accentColor: AppSettings["accentColor"];
  windowsAccentColor: string | null;
};

const DEFAULT_SETTINGS: AppSettings = {
  colorMode: "system",
  accentColor: { kind: "preset", value: "windy-teal" },
  editor: { executable: "", arguments: ["{path}"] },
};

function App() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [scans, setScans] = useState<Record<string, ScanState>>({});
  const [query, setQuery] = useState("");
  const [activeTag, setActiveTag] = useState<string | null>(null);
  const [dialog, setDialog] = useState<Dialog>(null);
  const [toast, setToast] = useState<string | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [settingsError, setSettingsError] = useState<string | null>(null);
  const [windowsAccentColor, setWindowsAccentColor] = useState<string | null>(null);
  const [windowsAccentError, setWindowsAccentError] = useState<string | null>(null);
  const [settingsPreviewState, setSettingsPreviewState] = useState<ThemeState | null>(null);
  const [settingsPreview, setSettingsPreview] = useState<SettingsPreview | null>(null);
  const [prefersDark, setPrefersDark] = useState(() =>
    typeof document !== "undefined" ? prefersDarkColorScheme(document.documentElement) : false,
  );
  const windowsAccentGeneration = useRef(createRequestGeneration());

  const selected = selectedId ? projects.find((p) => p.id === selectedId) ?? null : null;

  const loadSettings = useCallback(async () => {
    try {
      setSettings(await getSettings());
      setSettingsError(null);
    } catch (error) {
      setSettings({
        colorMode: DEFAULT_SETTINGS.colorMode,
        accentColor: { ...DEFAULT_SETTINGS.accentColor },
        editor: { ...DEFAULT_SETTINGS.editor, arguments: [...DEFAULT_SETTINGS.editor.arguments] },
      });
      setSettingsError(`Could not load settings: ${String(error)}`);
    }
  }, []);

  const loadWindowsAccent = useCallback(async () => {
    const request = windowsAccentGeneration.current.next();
    try {
      const value = await getWindowsAccentColor();
      if (!windowsAccentGeneration.current.isCurrent(request)) {
        return;
      }
      if (!isValidHexColor(value)) {
        throw new Error("Windows accent color returned an invalid #RRGGBB value");
      }
      setWindowsAccentColor(value);
      setWindowsAccentError(null);
    } catch (error) {
      if (!windowsAccentGeneration.current.isCurrent(request)) {
        return;
      }
      setWindowsAccentColor(null);
      setWindowsAccentError(`Could not read the Windows accent color: ${String(error)}`);
    }
  }, []);

  useEffect(() => {
    void loadSettings();
    void loadWindowsAccent();
  }, [loadSettings, loadWindowsAccent]);

  useEffect(() => {
    if (typeof window === "undefined" || typeof window.matchMedia !== "function") {
      return;
    }

    const media = window.matchMedia(PREFERS_DARK_QUERY);
    const onChange = (event: MediaQueryListEvent) => setPrefersDark(event.matches);
    setPrefersDark(media.matches);
    media.addEventListener("change", onChange);
    return () => media.removeEventListener("change", onChange);
  }, []);

  useEffect(() => {
    if (settingsPreview) {
      applyTheme(
        document.documentElement,
        settingsPreview.colorMode,
        settingsPreview.accentColor,
        settingsPreview.windowsAccentColor,
        prefersDark,
      );
      return;
    }

    const activeSettings = settings ?? DEFAULT_SETTINGS;
    applyTheme(
      document.documentElement,
      activeSettings.colorMode,
      activeSettings.accentColor,
      windowsAccentColor,
      prefersDark,
    );
  }, [settings, settingsPreview, settingsPreviewState, windowsAccentColor, prefersDark]);

  const showToast = useCallback((message: string) => {
    setToast(message);
  }, []);

  useEffect(() => {
    if (!toast) {
      return;
    }
    const t = setTimeout(() => setToast(null), 6000);
    return () => clearTimeout(t);
  }, [toast]);

  const scanOne = useCallback(async (project: Project) => {
    setScans((s) => ({ ...s, [project.id]: { status: "loading" } }));
    try {
      const data = await scanProject(project.path);
      setScans((s) => ({ ...s, [project.id]: { status: "ok", data } }));
    } catch (e) {
      setScans((s) => ({ ...s, [project.id]: { status: "error", message: String(e) } }));
    }
  }, []);

  useEffect(() => {
    getProjects()
      .then((list) => {
        setProjects(list);
        list.forEach((p) => scanOne(p));
      })
      .catch((e) => setLoadError(String(e)));
  }, [scanOne]);

  const tags = useMemo<TagCount[]>(() => {
    const counts = new Map<string, number>();
    for (const p of projects) {
      for (const t of p.tags) {
        counts.set(t, (counts.get(t) ?? 0) + 1);
      }
    }
    return [...counts.entries()]
      .sort((a, b) => a[0].localeCompare(b[0]))
      .map(([name, count]) => ({ name, count }));
  }, [projects]);

  const visible = useMemo(() => {
    const byTag = activeTag ? projects.filter((p) => p.tags.includes(activeTag)) : projects;
    return filterProjects(byTag, query);
  }, [projects, activeTag, query]);

  function handleCreated(project: Project) {
    setProjects((list) => [...list, project]);
    setDialog(null);
    scanOne(project);
  }

  function handleSaved(updated: Project) {
    setProjects((list) => list.map((p) => (p.id === updated.id ? updated : p)));
    setDialog(null);
    scanOne(updated);
  }

  async function handleDelete(project: Project) {
    try {
      await deleteProject(project.id);
      setProjects((list) => list.filter((p) => p.id !== project.id));
      setScans((s) => {
        const next = { ...s };
        delete next[project.id];
        return next;
      });
      if (selectedId === project.id) {
        setSelectedId(null);
      }
      setDialog(null);
    } catch (e) {
      showToast(String(e));
    }
  }

  function guarded(action: () => Promise<void>) {
    action().catch((e) => showToast(String(e)));
  }

  function openSettings() {
    setSettingsPreviewState(captureThemeState(document.documentElement));
    setSettingsPreview(null);
    setDialog({ kind: "settings" });
  }

  function closeSettings() {
    setSettingsPreview(null);
    setSettingsPreviewState(null);
    setDialog(null);
  }

  function restoreSettingsTheme() {
    if (settingsPreviewState) {
      restoreThemeState(document.documentElement, settingsPreviewState);
    }
  }

  const previewSettingsTheme = useCallback(
    (colorMode: AppSettings["colorMode"], accentColor: AppSettings["accentColor"], windowsColor: string | null) => {
      setSettingsPreview({ colorMode, accentColor, windowsAccentColor: windowsColor });
      previewTheme(document.documentElement, colorMode, accentColor, windowsColor);
    },
    [],
  );

  return (
    <div className="app">
      <Sidebar
        total={projects.length}
        tags={tags}
        activeTag={activeTag}
        onSelectTag={setActiveTag}
        onOpenSettings={openSettings}
      />

      <main className="main">
        {selected ? (
          <ProjectDetail
            project={selected}
            scan={scans[selected.id]}
            onBack={() => setSelectedId(null)}
            onRefresh={() => scanOne(selected)}
            onOpen={() => guarded(() => openProject(selected.path))}
            onRun={() => guarded(() => runProject(selected.path, selected.runCommand ?? ""))}
            onBuild={() => guarded(() => buildProject(selected.path, selected.buildCommand ?? ""))}
            onOpenInEditor={() => guarded(() => openInEditor(selected.path))}
            onEdit={() => setDialog({ kind: "edit", project: selected })}
            onDelete={() => setDialog({ kind: "delete", project: selected })}
          />
        ) : (
          <>
            <div className="toolbar">
              <input
                className="search"
                type="search"
                placeholder="Search name, description, tag or path"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                aria-label="Search projects"
              />
              <button className="btn btn-primary" onClick={() => setDialog({ kind: "add" })}>
                Add project
              </button>
            </div>

            {loadError ? (
              <div className="empty-state">
                <p>Could not load projects: {loadError}</p>
              </div>
            ) : projects.length === 0 ? (
              <div className="empty-state">
                <p>No projects yet.</p>
                <button className="btn btn-primary" onClick={() => setDialog({ kind: "add" })}>
                  Add your first project
                </button>
              </div>
            ) : visible.length === 0 ? (
              <div className="empty-state">
                <p>Nothing matches your search.</p>
              </div>
            ) : (
              <div className="card-grid">
                {visible.map((p) => (
                  <ProjectCard
                    key={p.id}
                    project={p}
                    scan={scans[p.id]}
                    onSelect={() => setSelectedId(p.id)}
                    onOpen={() => guarded(() => openProject(p.path))}
                    onRun={() => guarded(() => runProject(p.path, p.runCommand ?? ""))}
                    onBuild={() => guarded(() => buildProject(p.path, p.buildCommand ?? ""))}
                    onOpenInEditor={() => guarded(() => openInEditor(p.path))}
                    onEdit={() => setDialog({ kind: "edit", project: p })}
                    onDelete={() => setDialog({ kind: "delete", project: p })}
                    onRescan={() => scanOne(p)}
                  />
                ))}
              </div>
            )}
          </>
        )}
      </main>

      {dialog?.kind === "add" && (
        <AddProjectDialog
          projects={projects}
          onClose={() => setDialog(null)}
          onCreated={handleCreated}
        />
      )}
      {dialog?.kind === "edit" && (
        <EditProjectDialog
          project={dialog.project}
          onClose={() => setDialog(null)}
          onSaved={handleSaved}
        />
      )}
      {dialog?.kind === "delete" && (
        <ConfirmDialog
          title="Delete project"
          confirmLabel="Delete"
          message={
            <>
              Remove <strong>{dialog.project.name}</strong> from the list? The project
              directory on disk is never deleted.
            </>
          }
          onCancel={() => setDialog(null)}
          onConfirm={() => handleDelete(dialog.project)}
        />
      )}
      {dialog?.kind === "settings" && (
        <SettingsDialog
          settings={settings}
          windowsAccentColor={windowsAccentColor}
          settingsError={settingsError}
          windowsAccentError={windowsAccentError}
          onClose={closeSettings}
          onWindowsAccentResult={(color, error) => {
            windowsAccentGeneration.current.next();
            setWindowsAccentColor(color);
            setWindowsAccentError(error);
          }}
          onSaved={(saved) => {
            setSettings(saved);
            setSettingsError(null);
          }}
          onThemePreview={previewSettingsTheme}
          onThemeRestore={restoreSettingsTheme}
          onRetrySettings={() => void loadSettings()}
        />
      )}

      {toast && (
        <div className="toast" role="alert">
          {toast}
          <button className="icon-btn" aria-label="Dismiss" onClick={() => setToast(null)}>
            ✕
          </button>
        </div>
      )}
    </div>
  );
}

export default App;
