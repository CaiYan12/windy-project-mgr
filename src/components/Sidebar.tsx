// Sidebar（D9）：所有项目 + 标签过滤（从项目数据自动提取）+ 底部设置入口。

import { Icon } from "./Icon";

export interface TagCount {
  name: string;
  count: number;
}

export function Sidebar({
  total,
  tags,
  activeTag,
  onSelectTag,
  onOpenSettings,
}: {
  total: number;
  tags: TagCount[];
  activeTag: string | null;
  onSelectTag: (tag: string | null) => void;
  onOpenSettings: () => void;
}) {
  return (
    <aside className="sidebar">
      <div className="wordmark">
        <Icon name="windy" size={24} />
        <span>Windy</span>
      </div>

      <nav className="sidebar-nav">
        <button
          className={activeTag === null ? "nav-item active" : "nav-item"}
          onClick={() => onSelectTag(null)}
        >
          <span className="nav-item-label">
            <Icon name="layout-grid" size={16} />
            <span>All projects</span>
          </span>
          <span className="nav-count">{total}</span>
        </button>

        {tags.length > 0 && (
          <div className="nav-heading">
            <Icon name="tag" size={14} />
            <span>Tags</span>
          </div>
        )}
        {tags.map((t) => (
          <button
            key={t.name}
            className={activeTag === t.name ? "nav-item active" : "nav-item"}
            onClick={() => onSelectTag(activeTag === t.name ? null : t.name)}
          >
            <span className="nav-item-label">
              <Icon name="tag" size={15} />
              <span>{t.name}</span>
            </span>
            <span className="nav-count">{t.count}</span>
          </button>
        ))}
      </nav>

      <button className="nav-item sidebar-settings" onClick={onOpenSettings}>
        <span className="nav-item-label">
          <Icon name="settings" size={16} />
          <span>Settings</span>
        </span>
      </button>
    </aside>
  );
}
