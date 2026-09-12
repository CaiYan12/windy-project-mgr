// Edit Dialog（D10 单步）：可编辑全部字段（含 runCommand / buildCommand）。
// 路径查重以后端为唯一事实源（ADR 0007），保存前调用 `checkPathAvailable` 预校验。

import { useState } from "react";
import { Modal } from "./Modal";
import { Icon } from "./Icon";
import { parseTags, ProjectFields, type ProjectFormState } from "./ProjectFields";
import { checkPathAvailable, updateProject } from "../lib/api";
import type { Project } from "../types/project";

export function EditProjectDialog({
  project,
  onClose,
  onSaved,
}: {
  project: Project;
  onClose: () => void;
  onSaved: (updated: Project) => void;
}) {
  const [form, setForm] = useState<ProjectFormState>({
    name: project.name,
    path: project.path,
    description: project.description ?? "",
    tagsText: project.tags.join(", "),
    runCommand: project.runCommand ?? "",
    buildCommand: project.buildCommand ?? "",
  });
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  function patch(p: Partial<ProjectFormState>) {
    setForm((f) => ({ ...f, ...p }));
  }

  async function save() {
    setError(null);
    const path = form.path.trim();
    if (!path || !form.name.trim()) {
      setError("Path and name are required.");
      return;
    }
    setBusy(true);
    try {
      const availability = await checkPathAvailable(path, project.id);
      if (availability.status === "notAbsolute") {
        setError(`path must be absolute: ${path}`);
        return;
      }
      if (availability.status === "duplicate") {
        setError(`duplicate project path: ${availability.path} (edit the existing record instead)`);
        return;
      }
      const updated = await updateProject({
        ...project,
        name: form.name.trim(),
        path,
        description: form.description.trim() || undefined,
        tags: parseTags(form.tagsText),
        runCommand: form.runCommand.trim() || undefined,
        buildCommand: form.buildCommand.trim() || undefined,
      });
      onSaved(updated);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <Modal
      title="Edit project"
      onClose={onClose}
      footer={
        <>
          <button className="btn" onClick={onClose}>
            <Icon name="close" size={15} />
            <span>Cancel</span>
          </button>
          <button className="btn btn-primary" onClick={save} disabled={busy}>
            <Icon name="check-circle" size={15} />
            <span>Save changes</span>
          </button>
        </>
      }
    >
      <ProjectFields state={form} onChange={patch} showCommands />
      {error && (
        <p className="form-error">
          <Icon name="alert-triangle" size={16} />
          <span>{error}</span>
        </p>
      )}
    </Modal>
  );
}
