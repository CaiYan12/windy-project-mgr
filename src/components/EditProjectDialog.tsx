// Edit Dialog（D10 单步）：可编辑全部字段（含 runCommand / buildCommand）。

import { useState } from "react";
import { Modal } from "./Modal";
import { parseTags, ProjectFields, type ProjectFormState } from "./ProjectFields";
import { updateProject } from "../lib/api";
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
    if (!form.path.trim() || !form.name.trim()) {
      setError("Path and name are required.");
      return;
    }
    setBusy(true);
    try {
      const updated = await updateProject({
        ...project,
        name: form.name.trim(),
        path: form.path.trim(),
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
            Cancel
          </button>
          <button className="btn btn-primary" onClick={save} disabled={busy}>
            Save changes
          </button>
        </>
      }
    >
      <ProjectFields state={form} onChange={patch} showCommands />
      {error && <p className="form-error">{error}</p>}
    </Modal>
  );
}
