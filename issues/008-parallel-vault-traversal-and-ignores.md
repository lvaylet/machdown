## Parent PRD

`issues/prd.md`

## What to build

Implement high-performance parallel directory traversal and file discovery using multi-threaded worker pools. This handles scanning tens of thousands of notes across massive Obsidian vaults in milliseconds, while respecting `.gitignore`, `.machdownignore`, CLI `--ignore` globs, and automatically excluding internal vault configuration directories (`.obsidian/`).

Refer to sections:
- `Implementation Decisions -> Module Architecture -> Filesystem Traversal Module (machdown-fs)` in `issues/prd.md`
- `Testing Decisions -> Modules to be Tested -> machdown-fs Tests` in `issues/prd.md`

## Acceptance criteria

- [ ] Directory walking runs in parallel using `rayon` across all available CPU cores.
- [ ] Files and patterns specified in `.gitignore` are automatically ignored.
- [ ] Files and patterns specified in `.machdownignore` are respected.
- [ ] Custom CLI exclude patterns passed via `--ignore <GLOB>` are respected.
- [ ] Internal `.obsidian/` configuration folders are automatically ignored by default.
- [ ] Symlink traversal policy handles recursive loops safely.
- [ ] Automated tests verify traversal speed, ignore pattern compliance, and Obsidian folder exclusion.

## Blocked by

- Blocked by `issues/001-tracer-bullet-cli-and-whitespace.md`

## User stories addressed

- User story 40
- User story 41
- User story 42
- User story 50
