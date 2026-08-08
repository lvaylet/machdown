## Parent PRD

`issues/prd.md`

## What to build

Implement the configuration system and inline HTML comment suppression directives. This allows users to configure rule parameters via `.machdown.toml`, import existing `.markdownlint.{json,yaml,yml,rc}` configurations, apply nested directory overrides, and temporarily disable or re-enable specific rules within Markdown documents.

Refer to sections:
- `Implementation Decisions -> Module Architecture -> Configuration Module (machdown-config)` in `issues/prd.md`
- `Implementation Decisions -> Supported Rule Scope` in `issues/prd.md`
- `Testing Decisions -> Modules to be Tested -> machdown-config Tests` in `issues/prd.md`

## Acceptance criteria

- [ ] Native `.machdown.toml` configuration files are discovered and loaded from root or specified via `--config <path>`.
- [ ] Markdownlint configuration files (`.markdownlint.json`, `.markdownlint.yaml`, `.markdownlint.yml`, `.markdownlintrc`) are auto-detected and parsed with full parameter compatibility.
- [ ] Nested directory-level configuration overrides are merged hierarchically with parent configurations.
- [ ] Inline comment suppression directives (`<!-- machdown-disable MD013 -->`, `<!-- markdownlint-disable -->`) suppress matching violations for subsequent lines.
- [ ] Inline comment enable directives (`<!-- machdown-enable MD013 -->`, `<!-- markdownlint-enable -->`) re-enable rule checks for subsequent lines.
- [ ] Line-scoped suppression comments (e.g. `<!-- machdown-disable-line MD013 -->`, `<!-- machdown-disable-next-line MD013 -->`) are evaluated correctly.
- [ ] Automated tests verify config parsing, cascading inheritance, and inline suppression scenarios.

## Blocked by

- Blocked by `issues/006-convergence-engine-diff-and-dry-run.md`

## User stories addressed

- User story 35
- User story 36
- User story 37
- User story 38
- User story 39
