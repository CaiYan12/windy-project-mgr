// 路径纯逻辑测试（D10 name 自动填充 + D3 前端查重）。

import { describe, expect, it } from "vitest";
import { lastSegment, samePath } from "./paths";

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

describe("samePath", () => {
  it("matches identical paths case-sensitively", () => {
    expect(samePath("d:\\Dev\\app", "d:\\Dev\\app")).toBe(true);
  });

  it("matches case-insensitively (Windows semantics)", () => {
    expect(samePath("d:\\Dev\\app", "d:\\dev\\APP")).toBe(true);
  });

  it("ignores a trailing separator", () => {
    expect(samePath("d:\\Dev\\app\\", "d:\\Dev\\app")).toBe(true);
  });

  it("treats forward and back slashes as equal", () => {
    expect(samePath("d:/Dev/app", "d:\\Dev\\app")).toBe(true);
  });

  it("rejects different paths", () => {
    expect(samePath("d:\\Dev\\app", "d:\\Dev\\other")).toBe(false);
  });
});
