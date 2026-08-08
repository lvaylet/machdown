# Good and Bad Tests

## Good Tests

**Integration-style**: Test through real public interfaces and observable CLI/crate behavior, not internal helper logic.

```rust
// GOOD: Tests observable behavior through public crate API
#[test]
fn lints_and_fixes_trailing_whitespace() {
    let input = "# Heading   \nParagraph with trailing spaces   \n";
    let output = fix_document(input, &Config::default()).expect("fixing should succeed");
    assert_eq!(output, "# Heading\nParagraph with trailing spaces\n");
}

// GOOD: Tests external CLI behavior
#[test]
fn check_fails_on_unfixed_violations() {
    let mut cmd = Command::cargo_bin("machdown").unwrap();
    cmd.arg("check").arg("tests/fixtures/invalid_headings.md");
    cmd.assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("MD001"));
}
```

Characteristics:

- Tests behavior users/callers care about
- Uses public API and external CLI only
- Survives internal refactors (e.g. changing parser tokens or internal AST structs)
- Describes WHAT, not HOW
- One logical assertion per test

## Bad Tests

**Implementation-detail tests**: Coupled to internal struct fields or private methods.

```rust
// BAD: Tests private parser internals
#[test]
fn test_internal_token_cursor_index() {
    let mut parser = InternalLineParser::new("test");
    parser.step_byte_cursor();
    assert_eq!(parser.cursor_position(), 1); // Breaks if cursor mechanism refactors
}
```
