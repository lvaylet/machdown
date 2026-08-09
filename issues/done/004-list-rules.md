## Parent PRD

`issues/prd.md`

## What to build

Implement the AST list representations and the complete suite of unordered and ordered list rules. These enforce uniform bullet styles, proper indentation alignment, sequential ordered numbering, spaces after markers, and surrounding blank lines around list blocks.

Refer to sections:
- `Implementation Decisions -> Supported Rule Scope -> Lists` in `issues/prd.md`
- `Testing Decisions -> Modules to be Tested -> machdown-rules Tests` in `issues/prd.md`

## Acceptance criteria

- [ ] MD004 (unordered list bullet style consistency: `*`, `-`, `+`, or consistent) is checked and auto-fixed.
- [ ] MD005 (inconsistent indentation for list items at the same level) is checked.
- [ ] MD007 (unordered list indentation spaces, e.g. 2 spaces vs 4 spaces) is checked and auto-fixed.
- [ ] MD029 (ordered list item prefix numbering consistency, e.g. `1, 2, 3` or `1, 1, 1`) is checked and auto-fixed.
- [ ] MD030 (spaces after list markers) is checked and auto-fixed.
- [ ] MD032 (surrounding blank lines around list blocks) is checked and auto-fixed.
- [ ] Snapshot / fixture tests verify exact diagnostics for list violations and valid auto-fixed output.

## Blocked by

- Blocked by `issues/002-obsidian-syntax-preservation.md`

## User stories addressed

- User story 12
- User story 13
- User story 23
- User story 27
