## Parent PRD

`issues/prd.md`

## What to build

Implement the AST heading abstractions and the complete suite of heading-related linter and auto-fixer rules. These enforce semantic outline hierarchy, consistent heading styling, appropriate surrounding whitespace, and single top-level headings.

Refer to sections:
- `Implementation Decisions -> Supported Rule Scope -> Headings` in `issues/prd.md`
- `Testing Decisions -> Modules to be Tested -> machdown-rules Tests` in `issues/prd.md`

## Acceptance criteria

- [ ] MD001 (heading levels increment by only one level at a time) is checked.
- [ ] MD002 (first heading matches expected level) is checked.
- [ ] MD003 (heading style consistency between ATX `#` and Setext `===`/`---`) is checked and auto-fixed to configured style.
- [ ] MD018 / MD019 (spaces after hash in ATX headings) are checked and auto-fixed.
- [ ] MD020 / MD021 (spaces inside closed ATX heading hashes) are checked and auto-fixed.
- [ ] MD022 (surrounding blank lines around headings) is checked and auto-fixed.
- [ ] MD023 (heading starts at the beginning of the line without leading space) is checked and auto-fixed.
- [ ] MD025 (multiple top-level headings in the same document) is checked.
- [ ] Snapshot / fixture tests verify exact diagnostics for invalid heading files and correct transformation for auto-fixed files.

## Blocked by

- Blocked by `issues/002-obsidian-syntax-preservation.md`

## User stories addressed

- User story 9
- User story 10
- User story 11
- User story 19
- User story 20
- User story 21
