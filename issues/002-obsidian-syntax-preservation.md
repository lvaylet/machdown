## Parent PRD

`issues/prd.md`

## What to build

Extend `machdown-parser` and AST representations to natively recognize and preserve Obsidian-specific Markdown syntax without corruption during parsing or transformation. This ensures subsequent rules will not produce false positives or corrupt Obsidian constructs.

Refer to sections:
- `Implementation Decisions -> Module Architecture -> Parser Module (machdown-parser)` in `issues/prd.md`
- `Testing Decisions -> Modules to be Tested -> machdown-parser Tests` in `issues/prd.md`

## Acceptance criteria

- [ ] Parser recognizes YAML frontmatter blocks bounded by `---` at the start of files, preserving metadata keys and values intact.
- [ ] Parser recognizes Obsidian wiki-links (`[[Note]]` and `[[Note|Custom Display Text]]`) as distinct link nodes with accurate source spans.
- [ ] Parser recognizes Obsidian callout blocks (`> [!NOTE]`, `> [!WARNING]`, etc.) and their nested contents.
- [ ] Parser recognizes Obsidian block identifiers (e.g. `^block-id-123`) attached to paragraphs or list items without treating `^` as invalid punctuation.
- [ ] Parser recognizes inline math (`$E=mc^2$`) and display math blocks (`$$...$$`) without treating `$` as shell prompt or punctuation errors.
- [ ] Parser recognizes nested hashtags (e.g. `#project/alpha`) in prose without confusing them with ATX headings.
- [ ] Round-trip AST fidelity tests verify that Obsidian sample notes parse and reconstruct identical source text without data loss.

## Blocked by

- Blocked by `issues/001-tracer-bullet-cli-and-whitespace.md`

## User stories addressed

- User story 3
- User story 4
- User story 5
- User story 6
- User story 7
- User story 8
