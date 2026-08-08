## Parent PRD

`issues/prd.md`

## What to build

Implement the multi-pass iterative convergence engine, unified diff output, dry-run mode, and conflict resolution across multiple simultaneous rule fixes. This ensures that interdependent fixes resolve cleanly to a zero-diff state within a bounded number of iterations without corrupting files.

Refer to sections:
- `Implementation Decisions -> Module Architecture -> Fixer Module (machdown-fixer)` in `issues/prd.md`
- `Implementation Decisions -> Core CLI Contracts & Flags` in `issues/prd.md`
- `Testing Decisions -> Modules to be Tested -> machdown-fixer Tests` in `issues/prd.md`

## Acceptance criteria

- [ ] Fixer iteratively executes lint-and-fix passes up to `--max-passes <N>` (default: 10 passes) until document stabilizes with zero diff.
- [ ] Conflicting or overlapping text edits within a single pass are detected, ordered safely, and applied without data loss.
- [ ] Cyclic or non-converging edits terminate gracefully at the max passes limit and report remaining issues.
- [ ] `machdown fix --dry-run <path>` computes and validates all proposed fixes without writing any changes to disk.
- [ ] `machdown fix --diff <path>` generates and outputs a standard unified diff of all proposed modifications to stdout without modifying files on disk.
- [ ] Idempotency tests verify that running `machdown fix` on a previously fixed file produces zero changes and zero violations on the second run.

## Blocked by

- Blocked by `issues/003-heading-rules.md`
- Blocked by `issues/004-list-rules.md`
- Blocked by `issues/005-code-and-formatting-rules.md`

## User stories addressed

- User story 29
- User story 30
- User story 32
- User story 33
- User story 34
