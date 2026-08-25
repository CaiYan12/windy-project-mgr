// 两步 Add Dialog（D10）：Step 1 基础信息（路径选定后 name 自动取末段、可编辑；
// 路径校验 + 查重由后端承接），Step 2 启动脚本引导（D5，可跳过）。

import { useState } from "react";
import { open as openFilePicker } from "@tauri-apps/plugin-dialog";
import { Modal } from "./Modal";
import { parseTags, ProjectFields, type ProjectFormState } from "./ProjectFields";
import { createProject, listScripts } from "../lib/api";
import { lastSegment } from "../lib/paths";
import type { Project, StartupScript } from "../types/project";

const emptyForm: ProjectFormState = {
  name: "",
  path: "",
  description: "",
  tagsText: "",
  runCommand: "",
  buildCommand: "",
};

export function AddProjectDialog({
  onClose,
  onCreated,
}: {
  onClose: () => void;
  onCreated: (project: Project) => void;
}) {
  const [step, setStep] = useState<1 | 2>(1);
  const [form, setForm] = useState<ProjectFormState>(emptyForm);
  const [nameTouched, setNameTouched] = useState(false);
  const [scripts, setScripts] = useState<StartupScript[]>([]);
  const [picked, setPicked] = useState<StartupScript | null>(null);
  const [scriptsError, setScriptsError] = useState<string | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  function patch(p: Partial<ProjectFormState>) {
    if ("name" in p) {
      setNameTouched(true);
    }
    setForm((f) => {
      const next = { ...f, ...p };
      // D10：路径选定后 name 自动取末段填充，用户编辑过 name 后不再覆盖。
      if ("path" in p && !nameTouched) {
        next.name = lastSegment(p.path ?? "");
      }
      return next;
    });
  }

  async function next() {
    setError(null);
    if (!form.path.trim() || !form.name.trim()) {
      setError("Path and name are required.");
      return;
    }
    setBusy(true);
    try {
      const list = await listScripts(form.path.trim());
      setScripts(list);
      setPicked(null);
      setScriptsError(null);
      setSelected(list.length > 0 ? list[0].path : null);
      setStep(2);
    } catch (e) {
      const msg = String(e);
      if (msg.toLowerCase().includes("path not found")) {
        setError("Path not found: " + form.path.trim());
      } else {
        // 枚举失败不阻断添加：进入 Step 2 并提示。
        setScripts([]);
        setPicked(null);
        setScriptsError(msg);
        setSelected(null);
        setStep(2);
      }
    } finally {
      setBusy(false);
    }
  }

  async function submit() {
    setBusy(true);
    setError(null);
    try {
      const project = await createProject({
        name: form.name.trim(),
        path: form.path.trim(),
        description: form.description.trim() || undefined,
        tags: parseTags(form.tagsText),
        runCommand: selected ?? undefined,
        buildCommand: undefined,
      });
      onCreated(project);
    } catch (e) {
      setError(String(e));
      setStep(1);
    } finally {
      setBusy(false);
    }
  }

  // D5 自选脚本：文件选择器选文件，路径写入 runCommand；不复制、不解析。
  async function pickScript() {
    try {
      const file = await openFilePicker({
        multiple: false,
        filters: [{ name: "Startup scripts", extensions: ["bat", "cmd", "ps1"] }],
      });
      if (typeof file === "string") {
        const name = file.split(/[\\/]+/).pop() ?? file;
        setPicked({ name, path: file });
        setSelected(file);
      }
    } catch (e) {
      setScriptsError(String(e));
    }
  }

  const options: StartupScript[] = picked
    ? [...scripts, picked]
    : scripts;

  return (
    <Modal
      title={step === 1 ? "Add project" : "Add project · startup script"}
      onClose={onClose}
      footer={
        step === 1 ? (
          <>
            <button className="btn" onClick={onClose}>
              Cancel
            </button>
            <button className="btn btn-primary" onClick={next} disabled={busy}>
              Next
            </button>
          </>
        ) : (
          <>
            <button className="btn" onClick={() => setStep(1)} disabled={busy}>
              Back
            </button>
            <button className="btn btn-primary" onClick={submit} disabled={busy}>
              Add project
            </button>
          </>
        )
      }
    >
      {step === 1 ? (
        <>
          <ProjectFields state={form} onChange={patch} showCommands={false} nameAutoHint />
          {error && <p className="form-error">{error}</p>}
        </>
      ) : (
        <div className="script-step">
          <p className="script-hint">
            Pick a startup script to run this project. You can change this later.
          </p>
          {scriptsError && <p className="form-error">{scriptsError}</p>}
          {scripts.length === 0 && !scriptsError && (
            <p className="muted">No startup scripts found in the project root.</p>
          )}
          <div className="script-list">
            {options.map((s) => (
              <label key={s.path} className="script-option">
                <input
                  type="radio"
                  name="startup-script"
                  checked={selected === s.path}
                  onChange={() => setSelected(s.path)}
                />
                <span className="script-name">
                  {s.name}
                  {s.path === scripts[0]?.path && (
                    <span className="script-suggest">suggested</span>
                  )}
                </span>
                <span className="script-path">{s.path}</span>
              </label>
            ))}
            <label className="script-option">
              <input
                type="radio"
                name="startup-script"
                checked={selected === null}
                onChange={() => setSelected(null)}
              />
              <span className="script-name">None</span>
              <span className="script-path">Run command stays empty</span>
            </label>
          </div>
          <button className="btn small" onClick={pickScript}>
            Choose a script file…
          </button>
          {error && <p className="form-error">{error}</p>}
        </div>
      )}
    </Modal>
  );
}
