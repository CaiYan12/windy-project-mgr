# ADR 0005: Settings v2, editor discovery and accent colors

- Status: Accepted
- Date: 2026-08-27
- Decision owners: Windy Project Manager maintainers

## Context

The original settings stored a theme string and a free-form editor command. The UI presented static suggestions, could not distinguish installed editors, offered no executable picker, and always appended the project folder as an unconfigurable argument. That made folder-workspace support editor-dependent and caused folder-incompatible programs such as Notepad to fail.

## Decision

Settings v2 separates Color mode, Accent color, Editor profile and General information. The editor profile stores a resolved executable path plus one argument per line. A configured profile must contain exactly one `{path}` placeholder, which is replaced with the project directory. `.exe` files launch directly; `.cmd` and `.bat` files launch through `cmd.exe /d /c`.

The Editor page discovers only actual files from PATH, Windows uninstall registry entries and a bounded standard-path list. It supports VS Code, VS Code Insiders, Cursor, Windsurf, VSCodium and Zed. A PATH command without product evidence is shown as a PATH command rather than being mislabeled. Other selects only `.exe` files and warns that arbitrary executables may not support folder workspaces.

Accent color supports six application presets, the current Windows accent, and validated custom `#RRGGBB`. Windows accent lookup is performed at startup and when settings opens; failed lookup falls back to Windy teal with a notice. Accent variables are derived for primary, hover, soft, focus and on-accent states.

Valid v1 settings migrate automatically and atomically to v2. Existing project data and executable-relative data storage remain unchanged.

## Consequences

The settings schema and command surface grow, but editor behavior becomes explicit and testable. Arbitrary editors remain a manual compatibility boundary: the application can start the executable, but only manual acceptance can establish folder-workspace support. No editor process watcher or full-disk scanner is introduced.

## Alternatives rejected

- Static datalist presets: cannot prove installation or folder support.
- Passing the project folder as an implicit final argument: cannot configure editor-specific flags.
- Full-disk installation scan: too slow, invasive and unnecessary for the MVP.
- A new detection crate: `where.exe`, `reg.exe` and Rust standard APIs are sufficient.
