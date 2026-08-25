// 卡片数据组装纯逻辑（原始文档第 11 节降级文案 + D8 UI 口径）。
// 所有函数纯且可注入当前时间，便于测试。

import type { ActivityMetadata, GitMetadata } from "../types/project";

/** Git 行：`main · clean` / `main · 3 changed` / `detached@abc · unknown`。 */
export function gitLine(git: GitMetadata): string {
  if (git.status === "modified") {
    return `${git.branch} · ${git.changedFiles} changed`;
  }
  return `${git.branch} · ${git.status}`;
}

/** ahead/behind 仅在非零时显示（D8：无上游 = 0 且 UI 不显示）。 */
export function gitSyncLine(git: GitMetadata): string | null {
  const parts: string[] = [];
  if (git.ahead > 0) {
    parts.push(`ahead ${git.ahead}`);
  }
  if (git.behind > 0) {
    parts.push(`behind ${git.behind}`);
  }
  return parts.length > 0 ? parts.join(" · ") : null;
}

/** 活动行：无修改时间时降级为文档约定文案。 */
export function activityText(activity: ActivityMetadata, now: number): string {
  if (!activity.lastModifiedAt) {
    return "No recent activity";
  }
  return relativeTime(activity.lastModifiedAt, now);
}

/** ISO 时刻相对化：<60s just now、<60m Nm ago、<24h Nh ago、<=30d Nd ago，否则日期。 */
export function relativeTime(iso: string, now: number): string {
  const t = Date.parse(iso);
  if (Number.isNaN(t)) {
    return "No recent activity";
  }
  const secs = Math.max(0, Math.floor((now - t) / 1000));
  if (secs < 60) {
    return "just now";
  }
  const mins = Math.floor(secs / 60);
  if (mins < 60) {
    return `${mins}m ago`;
  }
  const hours = Math.floor(mins / 60);
  if (hours < 24) {
    return `${hours}h ago`;
  }
  const days = Math.floor(hours / 24);
  if (days <= 30) {
    return `${days}d ago`;
  }
  return iso.slice(0, 10);
}
