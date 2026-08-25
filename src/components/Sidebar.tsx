// Sidebar（D9）：所有项目 + 标签过滤（从项目数据自动提取）+ 底部设置入口。

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
        <span className="wind-mark" aria-hidden="true">
          <i />
          <i />
          <i />
        </span>
        Windy
      </div>

      <nav className="sidebar-nav">
        <button
          className={activeTag === null ? "nav-item active" : "nav-item"}
          onClick={() => onSelectTag(null)}
        >
          All projects
          <span className="nav-count">{total}</span>
        </button>

        {tags.length > 0 && <div className="nav-heading">Tags</div>}
        {tags.map((t) => (
          <button
            key={t.name}
            className={activeTag === t.name ? "nav-item active" : "nav-item"}
            onClick={() => onSelectTag(activeTag === t.name ? null : t.name)}
          >
            {t.name}
            <span className="nav-count">{t.count}</span>
          </button>
        ))}
      </nav>

      <button className="nav-item sidebar-settings" onClick={onOpenSettings}>
        Settings
      </button>
    </aside>
  );
}
