// Search 纯逻辑测试（D7：四字段大小写不敏感子串匹配，纯前端即时过滤）。

import { describe, expect, it } from "vitest";
import { filterProjects } from "./search";
import type { Project } from "../types/project";

function project(over: Partial<Project>): Project {
  return {
    id: "p1",
    name: "windy-project-mgr",
    path: "d:\\Dev\\windy-project-mgr",
    tags: [],
    createdAt: "2026-08-25T00:00:00Z",
    ...over,
  };
}

const all = [
  project({ id: "a", name: "Alpha", description: "First demo", tags: ["web"], path: "d:\\Dev\\alpha" }),
  project({ id: "b", name: "Beta", description: "Second tool", tags: ["cli", "rust"], path: "d:\\Work\\beta" }),
  project({ id: "c", name: "Gamma", tags: [], path: "d:\\Work\\gamma" }),
];

describe("filterProjects", () => {
  it("empty query returns everything", () => {
    expect(filterProjects(all, "")).toHaveLength(3);
    expect(filterProjects(all, "   ")).toHaveLength(3);
  });

  it("matches name case-insensitively", () => {
    expect(filterProjects(all, "ALPHA").map((p) => p.id)).toEqual(["a"]);
  });

  it("matches description", () => {
    expect(filterProjects(all, "second").map((p) => p.id)).toEqual(["b"]);
  });

  it("matches any tag", () => {
    expect(filterProjects(all, "Rust").map((p) => p.id)).toEqual(["b"]);
  });

  it("matches path", () => {
    expect(filterProjects(all, "work").map((p) => p.id)).toEqual(["b", "c"]);
  });

  it("no match yields empty result", () => {
    expect(filterProjects(all, "zzz")).toEqual([]);
  });
});
