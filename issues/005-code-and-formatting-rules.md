## Parent PRD

`issues/prd.md`

## What to build

Implement the remaining core formatting, code block, link, and emphasis rules. These enforce clean whitespace, tab conversions, dollar-sign stripping from shell snippets, language tags on code blocks, proper link spacing, and emphasis character consistency.

Refer to sections:
- `Implementation Decisions -> Supported Rule Scope -> Whitespace & Formatting` in `issues/prd.md`
- `Testing Decisions -> Modules to be Tested -> machdown-rules Tests` in `issues/prd.md`

## Acceptance criteria

- [ ] MD010 (hard tabs converted to spaces) is checked and auto-fixed.
- [ ] MD011 (reversed link syntax `(text)[url]` detected) is checked.
- [ ] MD012 (multiple consecutive blank lines collapsed into one) is checked and auto-fixed.
- [ ] MD013 (line length maximum threshold exceeded, excluding tables and URLs) is checked.
- [ ] MD014 (dollar signs in shell command code blocks) is checked and auto-fixed.
- [ ] MD027 / MD028 (blockquote spacing and blank lines) are checked and auto-fixed.
- [ ] MD031 (surrounding blank lines around fenced code blocks) is checked and auto-fixed.
- [ ] MD039 (spaces inside link text `[ text ](url)`) is checked and auto-fixed.
- [ ] MD040 (fenced code block missing language tag) is checked.
- [ ] MD047 (single trailing newline at EOF) is checked and auto-fixed.
- [ ] MD049 / MD050 (emphasis marker style consistency: `*` vs `_` for italic and bold) is checked and auto-fixed.
- [ ] Snapshot / fixture tests verify diagnostics and auto-fixes for all implemented rules.

## Blocked by

- Blocked by `issues/002-obsidian-syntax-preservation.md`

## User stories addressed

- User story 15
- User story 16
- User story 17
- User story 18
- User story 22
- User story 24
- User story 25
- User story 26
- User story 28
