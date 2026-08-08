# Product Requirements Document: machdown

## Problem Statement

Users managing large Markdown knowledge bases (such as Obsidian vaults), documentation repositories, and personal notes face accumulating inconsistencies, formatting defects, and Markdown style rule violations. Over time, these files suffer from irregular heading hierarchies, improper list indentations, inconsistent emphasis formatting, trailing whitespace, missing code block language tags, broken link syntax, and malformed frontmatter.

Existing Markdown linters are frequently slow when executed across vaults containing tens of thousands of notes, require heavy runtime environments (such as Node.js or Ruby), or lack automated fixing capabilities. Furthermore, existing linters often crash or produce false positive violations when encountering Obsidian-specific syntax extensions—including YAML frontmatter, `[[wiki-links]]`, callout blocks (`> [!NOTE]`), tags (`#tag`), math blocks (`$...$`), and block identifiers. Users need a single, fast, native command-line tool that can quickly detect style violations across entire vaults, report them with clear diagnostics, and automatically fix them safely without corrupting Obsidian-specific constructs or user content.

## Solution

`machdown` is a high-performance, native command-line interface (CLI) Markdown linter and auto-fixer written in Rust. It checks and automatically fixes Markdown files against the standard markdownlint rule catalog (MD001–MD056) while natively understanding and preserving Obsidian-specific syntax extensions.

`machdown` provides:

1. A **subcommand-driven CLI** (`check`, `fix`, `rules`) for scanning and fixing files, viewing unified diffs, and previewing fixes via dry-run mode.
2. An **extensible AST-based rule engine** capable of detecting style violations and applying lossless byte/line replacements.
3. An **iterative convergence fixer** that applies multi-pass non-conflicting fixes atomically to files while ensuring idempotency and data safety.
4. **Obsidian-aware parsing** that seamlessly handles YAML frontmatter, wiki-links, callouts, block references, tags, and math blocks without false positives.
5. **Flexible configuration & suppression** supporting native configuration files, markdownlint JSON/YAML compatibility, directory-scoped cascading configs, and inline HTML comment suppressions.
6. **High-performance parallel traversal** leveraging multithreading to check tens of thousands of vault notes in milliseconds while honoring `.gitignore`, `.machdownignore`, and internal vault folders (e.g., `.obsidian/`).
7. **Rich diagnostic reporting** supporting colored terminal output with code snippets and pointers, machine-readable JSON, and standard SARIF v2.1.0 for CI/CD integrations.

## User Stories

1. As a knowledge base maintainer, I want to run `machdown check .` across my entire Obsidian vault, so that I can immediately discover all Markdown formatting and style violations across all my notes.
2. As a knowledge base maintainer, I want to run `machdown fix .` on my vault, so that all auto-fixable Markdown style violations are corrected in-place automatically.
3. As an Obsidian user, I want `machdown` to recognize and preserve YAML frontmatter blocks, so that my note metadata is never mangled or flagged as invalid heading/paragraph structures.
4. As an Obsidian user, I want `machdown` to recognize `[[wiki-links]]` and `[[Note Title|Custom Display Text]]`, so that link-checking rules do not treat them as broken markdown links or raw bracket errors.
5. As an Obsidian user, I want `machdown` to recognize Obsidian callouts (`> [!NOTE]`, `> [!WARNING]`, etc.), so that blockquote rules treat them as valid callout constructs rather than malformed quotes.
6. As an Obsidian user, I want `machdown` to recognize block identifiers (such as `^block-id-123`), so that block reference anchors are preserved without raising punctuation or trailing character warnings.
7. As an Obsidian user, I want `machdown` to recognize inline math (`$E=mc^2$`) and block math (`$$...$$`), so that mathematical equations are not flagged as raw dollar-sign or punctuation violations.
8. As an Obsidian user, I want `machdown` to recognize nested hashtag tokens (such as `#project/alpha`), so that tags in prose are not confused with ATX heading syntax.
9. As a developer, I want `machdown` to check heading levels (MD001) to ensure headings only increment by one level at a time, so that document outline structure remains semantic and accessible.
10. As a developer, I want `machdown` to enforce that first headings match the top-level heading style (MD002), so that all documents maintain a consistent entry level.
11. As a developer, I want `machdown` to check and fix heading styles between ATX (`#`) and Setext (`===`/`---`) (MD003), so that all headings in a vault follow a single consistent visual style.
12. As a developer, I want `machdown` to check and fix unordered list bullet styles (`*`, `-`, `+`) (MD004), so that list symbols remain uniform throughout all documents.
13. As a developer, I want `machdown` to check and fix unordered list indentation levels (MD007), so that nested list items align with consistent space indentation.
14. As a developer, I want `machdown` to check and fix trailing whitespace at the ends of lines (MD009), so that unnecessary trailing characters are removed while preserving intentional double-space line breaks when configured.
15. As a developer, I want `machdown` to check and fix hard tab characters (MD010), so that tabs are replaced with consistent spaces according to document configuration.
16. As a developer, I want `machdown` to check and fix consecutive blank lines (MD012), so that multiple blank lines are collapsed into a single blank line.
17. As a developer, I want `machdown` to check maximum line length violations (MD013), so that paragraphs and lines do not exceed configured length thresholds unless specifically excluded (such as tables or URLs).
18. As a developer, I want `machdown` to check and fix dollar-signs in shell commands (MD014), so that copy-pastable code blocks do not contain extraneous prompt prefixes.
19. As a developer, I want `machdown` to check and fix surrounding blank lines around headings (MD022), so that headings are cleanly separated from surrounding body text.
20. As a developer, I want `machdown` to check and fix heading start/end spaces (MD023), so that heading titles do not have unintended whitespace padding after hashes.
21. As a developer, I want `machdown` to check and fix multiple top-level headings in the same document (MD025), so that each note has a single unique title heading.
22. As a developer, I want `machdown` to check and fix surrounding blank lines around blockquotes (MD027/MD028), so that blockquote blocks are visually separated from surrounding paragraphs.
23. As a developer, I want `machdown` to check and fix ordered list item prefix sequential numbering (MD029), so that ordered lists follow consistent numbering patterns (e.g., `1, 2, 3` or `1, 1, 1`).
24. As a developer, I want `machdown` to check and fix spaces inside link text and URLs (MD039), so that link text does not contain irregular leading or trailing whitespace.
25. As a developer, I want `machdown` to check and fix fenced code block language specifications (MD040), so that code blocks without a specified language are flagged for syntax highlighting.
26. As a developer, I want `machdown` to check and fix surrounding blank lines around fenced code blocks (MD031), so that code snippets are cleanly delimited from prose.
27. As a developer, I want `machdown` to check and fix blank lines around list items (MD032), so that lists are properly separated from surrounding text.
28. As a developer, I want `machdown` to check and fix emphasis marker styles (`*` vs `_`) (MD049/MD050), so that italics and bold text follow a unified Vault-wide standard.
29. As a developer, I want to run `machdown fix --dry-run [path]`, so that I can inspect what changes would be made without modifying any files on disk.
30. As a developer, I want to run `machdown fix --diff [path]`, so that I can see a unified colorized diff of every proposed correction before applying it.
31. As a developer, I want `machdown fix` to apply modifications atomically, so that a sudden process interruption or system crash never leaves a file half-written or corrupted.
32. As a developer, I want `machdown fix` to execute an iterative convergence loop, so that secondary violations introduced by primary fixes (such as spacing around newly formatted headings) are resolved within the same run.
33. As a developer, I want `machdown fix` to limit iterative passes to a bounded maximum (e.g., 10 passes), so that conflicting or non-converging rules cannot cause infinite loops.
34. As a developer, I want `machdown` to be idempotent, so that running `machdown fix` on a previously fixed file results in zero diffs and zero modifications.
35. As a team lead, I want to configure rule parameters via a root configuration file, so that our team's Markdown standards are versioned and enforced consistently.
36. As a developer migrating from existing JavaScript tooling, I want `machdown` to automatically detect and read `.markdownlint.json`, `.markdownlint.yaml`, and `.markdownlintrc` files, so that I do not need to rewrite our team's existing lint configurations.
37. As a developer, I want `machdown` to support nested directory-level configuration overrides, so that subdirectories (such as templates or imported archives) can have custom rule settings.
38. As a content author, I want to temporarily disable a rule for a specific line or block using inline HTML comments (e.g., `<!-- machdown-disable MD013 -->` or `<!-- markdownlint-disable -->`), so that intentional style exceptions are ignored without triggering errors.
39. As a content author, I want to re-enable rules after a disabled section (e.g., `<!-- machdown-enable MD013 -->`), so that linting resumes normally for the remainder of the document.
40. As a user with a large vault (50,000+ files), I want `machdown` to execute file traversal and lint checks in parallel using all CPU cores, so that complete vault checks finish in under a second.
41. As a user, I want `machdown` to automatically ignore files specified in `.gitignore` and `.machdownignore`, so that build artifacts, dependency directories, and transient files are not scanned.
42. As an Obsidian user, I want `machdown` to automatically ignore the `.obsidian/` configuration folder, so that internal workspace state files and plugin metadata are skipped by default.
43. As a CLI user, I want human-readable diagnostic output with file paths, line numbers, column numbers, rule identifiers, concise descriptions, and highlighted source code snippets with carets, so that I can quickly navigate to and understand issues.
44. As a CI/CD engineer, I want `machdown check --format sarif`, so that lint results can be ingested by GitHub Code Scanning and GitLab Code Quality dashboards natively.
45. As a script author, I want `machdown check --format json`, so that I can programmatically parse violations in custom automation workflows.
46. As a CI/CD engineer, I want standard exit codes (0 for clean, 1 for violations found, 2 for fatal configuration/IO errors), so that automated pipelines can easily determine pass/fail status.
47. As a developer, I want to run `machdown rules`, so that I can see a full list of supported rules, their configuration options, descriptions, and whether they support automatic fixing.
48. As a developer, I want to run `machdown rules [rule-id]`, so that I can view detailed documentation, examples of valid and invalid markdown, and auto-fix behavior for a specific rule.
49. As a developer, I want `machdown` to report rule violations with clickable terminal hyperlinks or direct documentation URLs, so that I can read the rule rationale with a single click.
50. As a developer, I want to target specific files or glob patterns (e.g., `machdown check "notes/**/*.md"`), so that I can lint only the files I am actively working on.

## Implementation Decisions

### Module Architecture

The system will be architected into seven distinct, deep, decoupled modules:

1. **Parser Module (`machdown-parser`)**:
   - **Role**: Builds a lossless Abstract / Concrete Syntax Tree (AST/CST) from raw Markdown text while preserving exact byte offsets, line numbers, and column positions for every node.
   - **Obsidian Extensions**: Custom lexing and parsing passes for YAML frontmatter headers, WikiLinks (`[[target]]`, `[[target|alias]]`), Callout headers (`> [!type]`), inline tags (`#tag`), block reference anchors (`^block-id`), and math blocks (`$...$`, `$$...$$`).
   - **Lossless Reconstruction**: Enables precise byte-range slice extraction and source-map tracking so edits can be applied without altering unrelated whitespace or comments.

2. **Rules Engine (`machdown-rules`)**:
   - **Role**: Defines the extensible rule interface, rule registry, rule configuration schemas, and execution dispatcher.
   - **Rule Abstraction**: Each rule implements a unified trait exposing metadata (ID, name, description, tags, default severity, fixability), a `check` method (evaluating the AST and document context to produce diagnostics), and a `fix` method (producing concrete text edit operations).
   - **Inline Suppression**: Evaluates comment directives (`machdown-disable`, `markdownlint-disable`, line-specific disables) during rule evaluation to filter out suppressed violations before diagnostic emission.

3. **Fixer Module (`machdown-fixer`)**:
   - **Role**: Manages the application of text edits, conflict resolution, iterative convergence, and safe file persistence.
   - **Convergence Loop**: Runs linting and fixing iteratively up to a configurable maximum pass limit (default: 10 passes) until the document converges to a zero-diff state or no further auto-fixable violations exist.
   - **Conflict Resolution**: Sorts and verifies non-overlapping byte replacements within each pass to prevent file corruption.
   - **Diff Generator**: Computes unified diff outputs comparing original and fixed documents for `--diff` and `--dry-run` modes.
   - **Atomic Persistence**: Writes modified documents to atomic temporary files before replacing the target file on disk.

4. **Configuration Module (`machdown-config`)**:
   - **Role**: Loads, validates, and merges configuration settings across multiple sources.
   - **Hierarchy & Formats**: Reads native TOML configuration files, standard markdownlint JSON (`.markdownlint.json`), YAML (`.markdownlint.yaml`, `.markdownlint.yml`), and `.markdownlintrc` formats.
   - **Cascading Merges**: Resolves root configuration files, folder-level nested configurations, and CLI argument overrides into an effective configuration for any given file path.

5. **Filesystem Traversal Module (`machdown-fs`)**:
   - **Role**: Discovers target Markdown files across directory trees with high-concurrency parallel traversal.
   - **Ignore Rules**: Honors `.gitignore` specifications, `.machdownignore` rules, command-line exclude globs, and automatically skips hidden Obsidian metadata directories (such as `.obsidian/`).
   - **Parallelism**: Distributes file discovery and parsing across worker thread pools to maximize multi-core CPU utilization.

6. **Reporter Module (`machdown-reporter`)**:
   - **Role**: Formats and outputs lint diagnostics and execution summaries.
   - **Output Formats**:
     - *Pretty Terminal Format*: ANSI-colorized output showing file path, line:column coordinates, severity, rule ID, diagnostic message, code snippet with carets, and documentation link.
     - *JSON Format*: Structured array of violation objects containing file, line, column, rule ID, rule name, severity, fixable status, and message.
     - *SARIF Format*: Standard SARIF v2.1.0 schema JSON compliant with static analysis tooling and GitHub Code Scanning.

7. **CLI Driver Module (`machdown-cli`)**:
   - **Role**: Command-line entry point, argument parsing, subcommand routing, progress reporting, and exit code management.
   - **Subcommands**:
     - `check`: Scans paths, reports violations, and returns status codes without altering files.
     - `fix`: Scans paths, applies automated fixes in-place (or displays diffs/dry-runs), and reports remaining unfixable issues.
     - `rules`: Lists available rules or inspects individual rule details and documentation.

### Core CLI Contracts & Flags

- **Commands**:
  - `machdown check [PATHS]... [FLAGS]`
  - `machdown fix [PATHS]... [FLAGS]`
  - `machdown rules [RULE_ID]`
- **Global / Subcommand Flags**:
  - `--config <PATH>`: Explicit path to configuration file.
  - `--diff`: Displays unified diff of proposed changes instead of writing to disk.
  - `--dry-run`: Performs lint and fix checks without modifying files on disk.
  - `--format <pretty|json|sarif>`: Specifies output format (default: `pretty`).
  - `--ignore <GLOB>`: Additional file/directory ignore patterns.
  - `--max-passes <N>`: Maximum number of iterative fix passes (default: 10).
  - `--quiet`: Suppresses informational headers and progress output, emitting only violations.
- **Exit Code Contract**:
  - `0`: Success (clean check, or all fixable violations resolved with no remaining errors).
  - `1`: Lint violations detected (in `check` mode, or unfixable violations remaining after `fix`).
  - `2`: Operational failure (invalid CLI flags, configuration parse error, or fatal I/O failure).

### Supported Rule Scope

The rule catalog establishes an extensible engine supporting the full MD001–MD056 specification, with initial high-priority auto-fixable rules implemented first:
- **Headings**: MD001 (heading increment), MD002 (first heading level), MD003 (heading style), MD018/MD019 (spaces after hash), MD020/MD021 (spaces inside closed hashes), MD022 (blank lines around headings), MD023 (heading start of line), MD025 (single top-level heading).
- **Lists**: MD004 (unordered list bullet style), MD005 (list item indentation consistency), MD007 (unordered list indentation spaces), MD029 (ordered list item prefix numbering), MD030 (spaces after list markers), MD032 (blank lines around lists).
- **Whitespace & Formatting**: MD009 (trailing whitespace), MD010 (hard tabs), MD011 (reversed link syntax), MD012 (consecutive blank lines), MD027/MD028 (blockquote spacing), MD031 (fenced code block blank lines), MD037/MD038 (spaces inside emphasis/code spans), MD039 (spaces inside link text), MD040 (fenced code language tag), MD047 (single trailing newline at EOF), MD049/MD050 (emphasis style consistency).

## Testing Decisions

### What Makes a Good Test

Tests must validate **external observable behavior** rather than internal implementation details:
1. **Rule Verification**: Tests should feed input Markdown text to the rule engine and assert the exact diagnostics produced (rule ID, line number, column number, and error message).
2. **Auto-Fix Verification**: Tests should take an invalid Markdown document, run the fixer, and assert that the output exactly matches an expected "golden" fixed document.
3. **Idempotency**: Every auto-fix test must run a second fix pass on its own output and assert that zero changes and zero violations are produced on the second run.
4. **Non-Regression on Vault Extensions**: Tests must verify that Obsidian constructs (frontmatter, wiki-links, callouts, tags, math blocks) are not mangled, corrupted, or falsely flagged by unrelated standard rules.
5. **CLI Contracts**: Integration tests must execute the binary as a separate process, testing argument combinations, configuration loading, formatted output (pretty, JSON, SARIF), file modification safety, and standard exit codes (`0`, `1`, `2`).

### Modules to be Tested

1. **`machdown-parser` Tests**:
   - Round-trip AST preservation tests with standard CommonMark fixtures.
   - Obsidian extension parser tests: YAML frontmatter parsing, `[[wiki-links]]` with aliases, `> [!callout]` blocks, `#nested/tags`, block IDs, inline and display math equations.
   - Source-span accuracy tests verifying that byte/line/column mappings accurately reflect original input locations.

2. **`machdown-rules` Tests**:
   - Dedicated fixture test suite for every implemented rule:
     - `valid.md`: Examples adhering to the rule that produce zero diagnostics.
     - `invalid.md`: Examples violating the rule that produce expected diagnostic locations and messages.
     - Rule configuration tests: Validating rule parameter variations (e.g., unordered list style `dash` vs `asterisk` vs `consistent`).
   - Inline suppression directive tests (`machdown-disable`, `markdownlint-disable`, block-scoped and line-scoped suppression).

3. **`machdown-fixer` Tests**:
   - Golden snapshot tests (`input.md` -> `expected_fixed.md`) for every auto-fixable rule.
   - Multi-pass convergence tests: Verifying that interdependent violations (e.g., list formatting + trailing spaces) resolve within minimal passes.
   - Convergence loop termination test: Verifying that cyclic or conflicting edits gracefully terminate at `--max-passes` without infinite loops.
   - Diff generation tests: Verifying standard unified diff output against known before/after file pairs.

4. **`machdown-config` Tests**:
   - Deserialization tests for `.machdown.toml`, `.markdownlint.json`, `.markdownlint.yaml`, and `.markdownlintrc`.
   - Hierarchical configuration cascading tests: Root config overridden by child directory config overridden by CLI flags.
   - Schema validation and error reporting tests for invalid configuration options.

5. **`machdown-fs` Tests**:
   - Directory traversal tests validating `.gitignore`, `.machdownignore`, and `.obsidian/` exclusion rules.
   - Symlink handling and recursive scanning tests across nested folder structures.

6. **`machdown-reporter` Tests**:
   - Snapshot tests for ANSI colorized terminal diagnostic output.
   - JSON output serialization tests validating structure against diagnostic schemas.
   - SARIF v2.1.0 schema compliance tests verifying schema validation against the official SARIF JSON schema.

7. **`machdown-cli` End-to-End Tests**:
   - `assert_cmd` test suite executing the compiled binary:
     - `machdown check` clean files -> exit code 0.
     - `machdown check` dirty files -> exit code 1 with diagnostic output.
     - `machdown fix` modifying files in-place -> exit code 0 on complete resolution.
     - `machdown fix --dry-run` leaving files unmodified on disk.
     - `machdown fix --diff` outputting unified diff to stdout.
     - Invalid flag / invalid config path -> exit code 2.

### Prior Art

- **markdownlint / markdownlint-cli2**: Industry-standard Node.js Markdown linter with MD001–MD056 rule specifications and test fixtures.
- **Ruff**: Python linter and auto-fixer in Rust serving as the architectural gold standard for high-performance AST parsing, multi-pass convergence fixing, unified diffs, and instant execution.
- **Biome / Prettier**: JavaScript/TypeScript formatters demonstrating robust lossless CST design, idempotency guarantees, and SARIF/JSON diagnostic reporters.

## Out of Scope

1. **Non-Markdown File Linting**: Linting non-Markdown files (e.g., pure YAML, JSON, CSS, JavaScript, or Canvas files) is out of scope.
2. **Grammar & Spell Checking**: Natural language grammar checking, spell checking, and prose style suggestions (such as Vale or LanguageTool features) are out of scope.
3. **Dead Link HTTP Checking**: Network requests to verify external HTTP/HTTPS URL availability are out of scope.
4. **Live Obsidian Plugin GUI**: Developing an in-app Obsidian desktop GUI plugin or Obsidian marketplace plugin is out of scope (can be built as a separate wrapper around `machdown` CLI in the future).
5. **Language Server Protocol (LSP) Daemon**: A persistent background LSP server binary is deferred to a subsequent milestone after the core CLI is complete.

## Further Notes

- **Performance Goal**: Target scanning and linting a 10,000-note Obsidian vault in less than 500 milliseconds on modern multi-core hardware.
- **Memory Safety & Reliability**: Zero unhandled panics; all parsing and fixing errors must be surfaced as structured diagnostics or graceful error reports.
- **Rule Extensibility**: The rule trait architecture is designed so new custom rules (including vault-specific link structure or tag naming rules) can be implemented with minimal boilerplate.
