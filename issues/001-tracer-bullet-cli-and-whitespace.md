## Parent PRD

`issues/prd.md`

## What to build

A fully functional end-to-end tracer bullet for `machdown`. This establishes the project skeleton with the core crates, a lossless line-oriented parser, the first rule MD009 (trailing whitespace check and fix), atomic file modification, basic ANSI colorized terminal diagnostic reporting, and CLI subcommands (`check` and `fix`) with standard exit codes.

Refer to sections:
- `Implementation Decisions -> Module Architecture` in `issues/prd.md`
- `Implementation Decisions -> Core CLI Contracts & Flags` in `issues/prd.md`

## Acceptance criteria

- [ ] Rust workspace initialized with core crates (`machdown-parser`, `machdown-rules`, `machdown-fixer`, `machdown-config`, `machdown-fs`, `machdown-reporter`, `machdown-cli`).
- [ ] `machdown-parser` parses document lines into a lossless representation preserving exact byte offsets, line numbers, and column numbers.
- [ ] Rule MD009 (trailing whitespace) is implemented with `check` and `fix` operations.
- [ ] `machdown check <file>` detects trailing whitespace, outputs diagnostic messages with line and column positions, and exits with code 1 if violations exist, or code 0 if clean.
- [ ] `machdown fix <file>` removes trailing whitespace in-place atomically using a temporary file replacement and exits with code 0 on clean fix.
- [ ] An unhandled error (such as missing file) exits with code 2.
- [ ] Automated tests verify MD009 linting, auto-fixing, idempotency, and CLI exit codes.

## Blocked by

None - can start immediately.

## User stories addressed

- User story 1
- User story 2
- User story 14
- User story 31
- User story 34
- User story 43
- User story 46
- User story 50
