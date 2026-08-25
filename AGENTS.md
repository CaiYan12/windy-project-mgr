# AGENTS.md

## **#0: Always be smart to use skills** like

- /grill-me on big changes sessions or any other you need to know.

- /frontend-design on any changes at frontend coding sessions.

- /animate, /find-animation-opportunities, /improve-animations, /review-aniamtions or other animation skills on animation designing or animation adjustments.

- /self-improvement when you make mistakes.

- /wayfinder on loose or unclear messages.

- /chinese-encoding on chinese language write-in sessions.

- /context7-mcp or other needing mcp servers.

  Read the skills, and involve the useful skills in your plan before you already know what to plan and to do.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:

- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:

- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:

- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:

- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:

```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

## 5. Session Closure — Self-Maintenance (Mandatory)

**Every session in this repository ends with a documentation-and-memory sync. No exceptions.**

Before closing any session, the agent MUST:

1. **Update `AGENTS.md`** to match reality: adjust every section affected by the session — Repository Status (phase, file inventory), Commands (once commands become runnable or change), Architecture (new decisions or structural changes), Execution Protocol, Hard Boundaries. Stale statements are bugs: claiming Rust installed when it isn't, or "code not yet initialized" after scaffolding exists, is a violation.
2. **Update `docs/PLAN.MD` section 0**: check verified subtask boxes and refresh the progress summary line (counts + date), per section 0.1.
3. **Update long-term memory** (`UpdateMemory`): create, update, or delete memories for persistent facts, decisions, pitfalls, or lessons surfaced this session; correct stale memories discovered during the session.
4. **Surgical self-maintenance only**: touch only sections that this session actually invalidated — no rewriting, reformatting, or "improving" untouched content.

The self-check question: "If a fresh agent starts tomorrow and reads only AGENTS.md + docs/PLAN.MD + memory, will it get the current truth?" If no, the session is not complete.

---

# WindyProjectMgr - Windy Project Manager

## Repository Status

This repository is **planning-complete, code not yet initialized** (Phase 0~1 done; progress tracked in `docs/PLAN.MD` section 0). It currently contains only documentation:

- `docs/Windy Project Manager - Primary Request&Plan Document.md` — original requirements and development rules (sections 1~28 baseline; sections 29~31 decision extensions). This is the authoritative spec.
- `docs/PLAN.MD` — self-contained executable plan: decisions D1~D13, Phase 0~14, acceptance criteria, and the checkbox progress tracker (section 0).
- `README.md` — containing basic project info and to-do list.
- `AGENTS.md` — basic agents' working needing messages, rules and guidelines
- `PROJECT_STATUS.md` — live development status plus the measured environment audit table (created in Phase 0)
- `CONTEXT.md` — domain glossary (D13, Phase 1)
- `CHANGELOG.md` — running record of completed changes (Phase 1 onward)
- `TESTING.md` — manual acceptance checklist skeleton, unchecked until Phase 12/14 (D13)
- `docs/adr/0001~0003` — ADRs for scan-data-memory-only (D2), detached run/build (D4), CSS-variable theming (D1)

Before the first of coding work, read `docs/PLAN.MD` first; it supersedes the primary document where they conflict.

## Commands

All commands below are **planned and not yet runnable** until Phase 2 (project scaffolding) completes. Package manager is pnpm by decision D11; never substitute npm/yarn.

```powershell
pnpm install                 # install frontend deps
pnpm tauri dev               # dev mode (Vite + Tauri window)
pnpm tauri build             # production bundle (Phase 13; sizes must be measured, not estimated)
cargo test                   # Rust unit + integration tests (src-tauri)
cargo test <test_name>       # run a single Rust test
pnpm vitest run              # frontend pure-logic tests only (search filter, path dedup normalization, card data assembly)
pnpm vitest run <file>       # run a single frontend test file
```

Rust is installed on the reference machine: rustup 1.29.0 with `stable-x86_64-pc-windows-msvc` (rustc / cargo 1.98.0, verified 2026-08-25 in Phase 0). Node v24.18.0, pnpm 10.26.2, Git 2.48.1, and VS2022 with VC x86/x64 are present; full audit table in `PROJECT_STATUS.md`. Note: shells opened before the install may need `%USERPROFILE%\.cargo\bin` on PATH or a restart.

## Architecture (Big Picture)

Windows-first local project indexer built with **React + TypeScript + Vite (frontend) over Tauri 2 IPC into Rust**. No local HTTP server, no database — persistence is two versioned JSON files in `%APPDATA%\windy-project-mgr\` (`projects.json`, `settings.json`), both written via temp-file-then-rename so failures never corrupt data.

Cross-cutting decisions that shape implementation (full text in `docs/PLAN.MD` section 4):

- **Runtime vs persisted split**: `Project` (persisted) holds only id/name/path/description/tags/runCommand/buildCommand/createdAt. All scan output (`ProjectMetadata`: projectType, techStack, git, activity) is computed at runtime and lives **only in frontend memory**. Startup renders card skeletons immediately, then concurrently calls `scan_project` for every project and fills cards as results arrive. No TTL, no disk cache.
- **Scanner pipeline**: path → project-type detection → tech-stack detection → git scanner → activity scanner, using only flat file-feature checks (package.json, Cargo.toml, etc.). No AST analysis, no recursive deep scans. Root-dir startup-script enumeration (`*.bat`/`*.cmd`/`*.ps1`, sorted start > run > alphabetical) feeds the Add Dialog step 2.
- **Git scanner shells out to system Git CLI**; it must never run `git fetch` (fully offline). Edge semantics: no upstream → ahead/behind = 0 and hidden in UI; empty repo → lastCommit = null; detached HEAD → branch = `detached@<shorthash>`; recentCommits capped at 10.
- **Run/Build/Open-in-editor are detached launches**: prefer `wt.exe`, fall back to `powershell -NoExit`, cwd = project path. The app reports only launch success/failure — it never captures exit codes or output, so "Build failed" in acceptance tests means the launch action failed, not the command.
- **Path dedup on add**: normalize to absolute path, compare case-insensitively (Windows semantics); duplicates are rejected with a message, never merged.
- **Theming**: single global stylesheet, all colors/spacing via CSS variables, light/dark sets switched by root `data-theme`; default follows system, manual choice persists to `settings.json` (`theme` field). Future theme presets = new variable sets, no mechanism change.
- **Tauri command surface is intentionally minimal**: CRUD (5) + `scan_project` + `open_project`/`run_project`/`build_project` + `get_settings`/`update_settings`/`open_in_editor`. Do not add Service/Controller/Repository layers unless code size proves the need.

## Execution Protocol (Mandatory)

- `docs/PLAN.MD` section 0 contains the 34-task checkbox tracker. After a subtask is **completed and verified**, check its box and update the progress summary line (counts + date) in the same edit. Never pre-check unverified work; blocked tasks stay unchecked and get evidence logged in `PROJECT_STATUS.md` Blocked section.
- Maintain `PROJECT_STATUS.md` (phase/status/tests/build/next-step) and `CHANGELOG.md` continuously; report each finished Phase in the structured format from primary document section 18.
- Evidence-first: verify environment, files, and command output before asserting anything; never claim an unexecuted command or unrun test as done.
- Tests before implementation for the data layer (Phase 4). Rust tests use temp dirs and **real temporary git repositories**, not mocks alone.
- **Session closure is mandatory**: rule #5 applies to every session — sync `AGENTS.md`, `docs/PLAN.MD` section 0, and long-term memory before ending. Skipping it leaves the next agent misinformed.

## Hard Boundaries (MVP Exclusions)

Do not implement, regardless of apparent need: Electron, embedded/implemented Git, terminal emulator, local HTTP server, AI agents, task/issue/todo systems, cloud sync, accounts, Docker management, full-disk auto-scan, plugin marketplaces or DI frameworks, SSH/remote transport, any UI or state-management framework beyond React + plain CSS variables. Every new dependency must justify its value; prefer Rust std and platform APIs. Deleting a project record must never delete the project directory.
