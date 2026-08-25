// 设置入口（D9 底部入口）：本阶段为只读状态呈现。
// 主题手动选择与持久化属 Phase 11（D1），编辑器命令属 Phase 10（D6）。

import { Modal } from "./Modal";

export function SettingsDialog({ onClose }: { onClose: () => void }) {
  return (
    <Modal
      title="Settings"
      onClose={onClose}
      footer={
        <button className="btn btn-primary" onClick={onClose}>
          Close
        </button>
      }
    >
      <div className="settings-rows">
        <div className="settings-row">
          <span className="settings-name">Theme</span>
          <span className="muted">Follows your system theme</span>
        </div>
        <div className="settings-row">
          <span className="settings-name">Editor</span>
          <span className="muted">Not configured</span>
        </div>
      </div>
    </Modal>
  );
}
