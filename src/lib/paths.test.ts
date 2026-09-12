// 路径纯逻辑测试（D10 name 自动填充）。D3 路径查重已移往后端（ADR 0007）。

import { describe, expect, it } from "vitest";
import { lastSegment } from "./paths";

describe("lastSegment", () => {
  it("takes the last segment of windows paths", () => {
    expect(lastSegment("d:\\Dev\\windy-project-mgr")).toBe("windy-project-mgr");
  });

  it("ignores trailing separators", () => {
    expect(lastSegment("d:\\Dev\\app\\")).toBe("app");
    expect(lastSegment("/home/user/app/")).toBe("app");
  });

  it("handles forward slashes", () => {
    expect(lastSegment("d:/Dev/app")).toBe("app");
  });

  it("returns empty for empty or separator-only input", () => {
    expect(lastSegment("")).toBe("");
    expect(lastSegment("\\")).toBe("");
  });
});
