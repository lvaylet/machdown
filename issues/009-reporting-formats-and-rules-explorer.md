## Parent PRD

`issues/prd.md`

## What to build

Implement structured machine-readable reporting formats (`--format json` and standard SARIF v2.1.0 `--format sarif`), rich terminal hyperlinking to documentation URLs, and the interactive `machdown rules` discovery subcommand for inspecting available rules, parameters, and auto-fix capabilities.

Refer to sections:
- `Implementation Decisions -> Module Architecture -> Reporter Module (machdown-reporter)` in `issues/prd.md`
- `Implementation Decisions -> Core CLI Contracts & Flags` in `issues/prd.md`
- `Testing Decisions -> Modules to be Tested -> machdown-reporter Tests` in `issues/prd.md`

## Acceptance criteria

- [ ] `machdown check --format json` emits structured JSON array of diagnostics with file, line, column, rule ID, severity, message, and fixable flag.
- [ ] `machdown check --format sarif` outputs standard SARIF v2.1.0 compliant JSON schema for GitHub Code Scanning / GitLab Code Quality integration.
- [ ] Pretty terminal reporter outputs clickable hyperlinks or direct URLs to rule documentation.
- [ ] `machdown rules` lists all supported rules with their IDs, names, descriptions, and auto-fix availability.
- [ ] `machdown rules <RULE_ID>` displays detailed documentation, configuration options, valid/invalid code examples, and auto-fix details for the specified rule.
- [ ] Automated tests validate SARIF JSON schema compliance, JSON output structure, and `machdown rules` CLI output.

## Blocked by

- Blocked by `issues/007-configuration-and-inline-suppression.md`
- Blocked by `issues/008-parallel-vault-traversal-and-ignores.md`

## User stories addressed

- User story 44
- User story 45
- User story 47
- User story 48
- User story 49
