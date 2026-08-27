// 卡片数据组装纯逻辑测试（原始文档第 11 节降级文案 + D8 UI 口径）。

import { describe, expect, it } from "vitest";
import {
  activityText,
  formatDateTime,
  gitLine,
  gitSyncLine,
  relativeTime,
  shortHash,
} from "./cards";
import type { ActivityMetadata, GitMetadata } from "../types/project";

function git(over: Partial<GitMetadata>): GitMetadata {
  return {
    branch: "main",
    status: "clean",
    changedFiles: 0,
    ahead: 0,
    behind: 0,
    lastCommit: null,
    recentCommits: [],
    ...over,
  };
}

describe("gitLine", () => {
  it("clean repo shows branch and clean", () => {
    expect(gitLine(git({}))).toBe("main · clean");
  });

  it("modified repo shows changed file count", () => {
    expect(gitLine(git({ status: "modified", changedFiles: 3 }))).toBe("main · 3 changed");
    expect(gitLine(git({ status: "modified", changedFiles: 1 }))).toBe("main · 1 changed");
  });

  it("unknown status is shown as such", () => {
    expect(gitLine(git({ status: "unknown" }))).toBe("main · unknown");
  });
});

describe("gitSyncLine", () => {
  it("no upstream (0/0) is hidden", () => {
    expect(gitSyncLine(git({}))).toBeNull();
  });

  it("ahead only", () => {
    expect(gitSyncLine(git({ ahead: 2 }))).toBe("ahead 2");
  });

  it("behind only", () => {
    expect(gitSyncLine(git({ behind: 1 }))).toBe("behind 1");
  });

  it("both directions", () => {
    expect(gitSyncLine(git({ ahead: 1, behind: 2 }))).toBe("ahead 1 · behind 2");
  });
});

describe("activityText", () => {
  const now = Date.parse("2026-08-25T12:00:00Z");

  it("no activity degrades to the documented text", () => {
    const a: ActivityMetadata = { lastModifiedAt: null, lastScannedAt: "x" };
    expect(activityText(a, now)).toBe("No recent activity");
  });

  it("renders relative time", () => {
    const a: ActivityMetadata = {
      lastModifiedAt: "2026-08-22T12:00:00Z",
      lastScannedAt: "x",
    };
    expect(activityText(a, now)).toBe("3d ago");
  });
});

describe("relativeTime", () => {
  const now = Date.parse("2026-08-25T12:00:00Z");

  it("buckets minutes, hours and days", () => {
    expect(relativeTime("2026-08-25T11:59:30Z", now)).toBe("just now");
    expect(relativeTime("2026-08-25T11:30:00Z", now)).toBe("30m ago");
    expect(relativeTime("2026-08-25T06:00:00Z", now)).toBe("6h ago");
    expect(relativeTime("2026-08-18T12:00:00Z", now)).toBe("7d ago");
  });

  it("falls back to the date beyond 30 days", () => {
    expect(relativeTime("2026-05-01T00:00:00Z", now)).toBe("2026-05-01");
  });

  it("unparsable input degrades to the no-activity text", () => {
    expect(relativeTime("not-a-date", now)).toBe("No recent activity");
  });
});

describe("shortHash", () => {
  it("takes the first 7 chars of a full 40-char hash", () => {
    expect(shortHash("a".repeat(40))).toBe("aaaaaaa");
    expect(shortHash("0123456789abcdef0123456789abcdef01234567")).toBe("0123456");
  });

  it("returns short inputs unchanged", () => {
    expect(shortHash("abc123")).toBe("abc123");
    expect(shortHash("1234567")).toBe("1234567");
  });

  it("returns empty string for empty input", () => {
    expect(shortHash("")).toBe("");
  });
});

describe("formatDateTime", () => {
  it("formats a UTC ISO instant as local YYYY-MM-DD HH:mm", () => {
    const out = formatDateTime("2026-08-26T06:16:30Z");
    // 当地时区下字段与 Date 重建一致（避免硬编码受机器时区/夏令时影响）
    const d = new Date("2026-08-26T06:16:30Z");
    const pad = (n: number) => String(n).padStart(2, "0");
    expect(out).toBe(
      `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`,
    );
    expect(out).toMatch(/^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$/);
  });

  it("returns the input unchanged when unparsable", () => {
    expect(formatDateTime("not-a-date")).toBe("not-a-date");
  });
});
