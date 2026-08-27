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

This repository has completed MVP implementation, integration, production build, and final acceptance (15 / 15 Phases, 34 / 34 subtasks; progress tracked in `docs/PLAN.MD` section 0). Top-level layout: documentation (`docs/`, `CONTEXT.md`, `TESTING.md`, `PROJECT_STATUS.md`, `CHANGELOG.md`) plus the app code — `src/` (React 19 + TS 5.8 + Vite 7 frontend: Dashboard and Detail implemented — `components/` Sidebar/Card/Dialogs, `pages/ProjectDetail.tsx` (seven sections + degraded states), `lib/` api+search+cards+paths+theme+settingsUi with v2 settings types/helpers and 64 Vitest tests, `types/`; theme system with `data-theme` and accent CSS variables applies persisted v2 settings, while `SettingsDialog` edits `settings.editor`), `src-tauri/` (Tauri 2.11.5, identifier `com.windy.project-mgr`; data layer in `src/project/`, CRUD commands in `src/commands/project.rs`, scan commands in `src/commands/scan.rs`, action/settings commands in `src/commands/actions.rs`, system commands in `src/commands/system.rs`, scanner in `src/scanner/mod.rs`, git scanner in `src/git/mod.rs` (system Git CLI, offline per D8), detached launcher in `src/launch/mod.rs` (D4), executable-relative portable data directory (D14), 148 passing Cargo tests in the 2026-08-27 full run), and root Vite/TS configs. `build.bat` / `build.ps1` produce the green directory and ZIP. Documentation inventory:

- `docs/Windy Project Manager - Primary Request&Plan Document.md` — original requirements and development rules (sections 1~28 baseline; sections 29~31 decision extensions). This is the authoritative spec.
- `docs/PLAN.MD` — self-contained executable plan: decisions D1~D14, Phase 0~14, acceptance criteria, and the checkbox progress tracker (section 0).
- `README.md` — containing basic project info and to-do list.
- `AGENTS.md` — basic agents' working needing messages, rules and guidelines
- `PROJECT_STATUS.md` — live development status plus the measured environment audit table (created in Phase 0)
- `CONTEXT.md` — domain glossary (D13, Phase 1)
- `CHANGELOG.md` — running record of completed changes (Phase 1 onward)
- `TESTING.md` — manual acceptance checklist for Phase 12/14, final acceptance completed (D13)
- `docs/adr/0001~0005` — ADRs for scan-data-memory-only (D2), detached run/build (D4), CSS-variable theming and accent colors (D1), portable data + green ZIP (D14), and Settings v2 editor discovery

Before the first of coding work, read `docs/PLAN.MD` first; it supersedes the primary document where they conflict.

## Commands

All commands below are **runnable** since Phase 2 (project scaffolding) completed. Package manager is pnpm by decision D11; never substitute npm/yarn. Note: pnpm build scripts are allowlisted via `pnpm.onlyBuiltDependencies` in `package.json` (esbuild).

```powershell
pnpm install                 # install frontend deps
pnpm tauri dev               # dev mode (Vite + Tauri window)
.\build.bat                  # green ZIP release: build/win-unpacked + versioned release directory/zip
pnpm tauri build --no-bundle # underlying release EXE build used by build.bat
cargo test                   # Rust unit + integration tests (src-tauri)
cargo test <test_name>       # run a single Rust test
pnpm vitest run              # frontend pure-logic/static-contract tests (search, cards, theme, settings, and path helpers)
pnpm vitest run <file>       # run a single frontend test file
pnpm test                    # alias of `pnpm vitest run`
```

All rows are runnable now; the `vitest` rows became runnable in Phase 8 (vitest 4 is a devDependency; 64 pure/static-logic tests, no component render tests per D12).

Cargo dev/test profiles use `debug = "line-tables-only"` in `src-tauri/Cargo.toml` to retain source line information while limiting Windows PDB and `target/` growth. A verified clean dev start produced 2.169 GiB; the pre-D14 97-test baseline raised it to 2.448 GiB (2026-08-26). The current suite contains 148 Rust tests in the 2026-08-27 full run.

Rust is installed on the reference machine: rustup 1.29.0 with `stable-x86_64-pc-windows-msvc` (rustc / cargo 1.98.0, verified 2026-08-25 in Phase 0). Node v24.18.0, pnpm 10.26.2, Git 2.48.1, and VS2022 with VC x86/x64 are present; full audit table in `PROJECT_STATUS.md`. Note: shells opened before the install may need `%USERPROFILE%\.cargo\bin` on PATH or a restart.

## Architecture (Big Picture)

Windows-first local project indexer built with **React + TypeScript + Vite (frontend) over Tauri 2 IPC into Rust**. No local HTTP server, no database — persistence is two versioned JSON files in `<current executable directory>\data\` (`projects.json`, `settings.json`), both written via temp-file-then-rename so failures never corrupt data. There is no AppData fallback or automatic migration (D14).

Cross-cutting decisions that shape implementation (full text in `docs/PLAN.MD` section 4):

- **Runtime vs persisted split**: `Project` (persisted) holds only id/name/path/description/tags/runCommand/buildCommand/createdAt. All scan output (`ProjectMetadata`: projectType, techStack, git, activity) is computed at runtime and lives **only in frontend memory**. Startup renders card skeletons immediately, then concurrently calls `scan_project` for every project and fills cards as results arrive. No TTL, no disk cache. The Detail page (Phase 9) consumes the same in-memory cache; its Refresh button re-runs `scan_project` for that single project.
- **Scanner pipeline**: path → project-type detection → tech-stack detection → git scanner → activity scanner, using only flat file-feature checks (package.json, Cargo.toml, etc.). No AST analysis, no recursive deep scans. Root-dir startup-script enumeration (`*.bat`/`*.cmd`/`*.ps1`, sorted start > run > alphabetical) feeds the Add Dialog step 2.
- **Git scanner shells out to system Git CLI**; it must never run `git fetch` (fully offline). Edge semantics: no upstream → ahead/behind = 0 and hidden in UI; empty repo → lastCommit = null; detached HEAD → branch = `detached@<shorthash>`; recentCommits capped at 10.
- **Run/Build/Open-in-editor are detached launches**: prefer `wt.exe`, fall back to `powershell -NoExit`, cwd = project path. The app reports only launch success/failure — it never captures exit codes or output, so "Build failed" in acceptance tests means the launch action failed, not the command.
- **Path dedup on add**: normalize to absolute path, compare case-insensitively (Windows semantics); duplicates are rejected with a message, never merged.
- **Theming**: single global stylesheet, all colors/spacing via CSS variables, light/dark sets switched by root `data-theme` (`system` follows `prefers-color-scheme`; `light`/`dark` force a palette). Manual color mode, tagged accent selection and editor profile persist independently in v2 `settings.json` fields (`colorMode`, `accentColor`, `editor`); the app reads them on startup and applies the effective theme. Theme presets = future new variable sets, no mechanism change.
- **Portable data + green release (D14)**: `app_data_dir()` resolves only `<current EXE directory>\data`; all command handlers retain testable `*_in(data_dir)` cores. Never add an AppData fallback or automatic migration. Dev data is therefore under `src-tauri\target\debug\data` and `cargo clean` deletes it. `build.bat` must remain the release entrypoint: it delegates to PowerShell 7, runs `pnpm tauri build --no-bundle`, and produces only `build\win-unpacked`, a versioned `release\` directory, and its ZIP—no MSI/NSIS.
- **Tauri command surface is intentionally minimal**: CRUD (5) + `scan_project` + `list_scripts` (added Phase 8: Add Dialog Step 2 needs script enumeration for unregistered paths, D5; justified in `src/commands/scan.rs`) + `open_project`/`run_project`/`build_project` + `get_settings`/`update_settings`/`open_in_editor` + `detect_editors`/`get_windows_accent_color`/`get_app_info` (system commands added for settings v2; §2.5 surface now complete). Do not add Service/Controller/Repository layers unless code size proves the need. Current wiring: all 16 commands are registered (Phase 5/8/10 plus the three Task 2 system commands); the template `greet` command was removed with the template UI. Frontend uses only React built-in hooks + a single CSS-variable stylesheet (ADR 0003); Dashboard↔Detail view switching is React state (`selectedId` in `App.tsx`, no router library); theme `data-theme` and accent variables are applied from persisted v2 settings at startup and on save (SettingsDialog).

## Execution Protocol (Mandatory)

- Current command-registration audit (2026-08-27): `src-tauri/src/lib.rs` registers 16 Tauri commands, including editor discovery, Windows accent, and app-info commands; Task 3 launcher fix3 and the Stage 9 custom-executable visibility fix are verified by focused and full tests.
- Task 5 / Stage 6 strict re-review was closed by fix2 and `task-5-rereview2.md` (PASS). Stage 8 real-window acceptance and Stage 9 custom-executable regression evidence are recorded in `task-7-report.md` and `task-8-bugfix-report.md`; the user subsequently confirmed the remaining Phase 14 manual acceptance items passed.

- `docs/PLAN.MD` section 0 contains the 34-task checkbox tracker. After a subtask is **completed and verified**, check its box and update the progress summary line (counts + date) in the same edit. Never pre-check unverified work; blocked tasks stay unchecked and get evidence logged in `PROJECT_STATUS.md` Blocked section.
- Maintain `PROJECT_STATUS.md` (phase/status/tests/build/next-step) and `CHANGELOG.md` continuously; report each finished Phase in the structured format from primary document section 18.
- Evidence-first: verify environment, files, and command output before asserting anything; never claim an unexecuted command or unrun test as done.
- Tests before implementation for the data layer (Phase 4). Rust tests use temp dirs and **real temporary git repositories**, not mocks alone.
- **Session closure is mandatory**: rule #5 applies to every session — sync `AGENTS.md`, `docs/PLAN.MD` section 0, and long-term memory before ending. Skipping it leaves the next agent misinformed.

## Hard Boundaries (MVP Exclusions)

Do not implement, regardless of apparent need: Electron, embedded/implemented Git, terminal emulator, local HTTP server, AI agents, task/issue/todo systems, cloud sync, accounts, Docker management, full-disk auto-scan, plugin marketplaces or DI frameworks, SSH/remote transport, any UI or state-management framework beyond React + plain CSS variables. Every new dependency must justify its value; prefer Rust std and platform APIs. Deleting a project record must never delete the project directory.
