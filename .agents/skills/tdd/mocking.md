# When to Mock

Mock at **system boundaries** only:

- External Network / HTTP APIs
- Clock / Time sources (if deterministic timestamps are required)
- OS System Environment (when simulating missing permissions or unusual IO errors)

Don't mock:

- Your own structs, traits, or crate modules
- Internal collaborators or parser stages
- Anything you control directly in the codebase

## Designing for Testability in Rust

At system boundaries, design interfaces using standard Rust idioms:

**1. Use Traits / Generics for Dependency Injection**

Pass external dependencies as trait implementations or generic types rather than hardcoding concrete IO:

```rust
// Easy to test / mock at boundaries
pub fn write_report<W: std::io::Write>(writer: &mut W, report: &Report) -> std::io::Result<()> {
    writeln!(writer, "{}", report.render())
}

// Hard to test without filesystem side effects
pub fn write_report(path: &Path, report: &Report) -> std::io::Result<()> {
    std::fs::write(path, report.render())
}
```

**2. Prefer In-Memory Fixtures & Temporary Directories**

In Rust, use `tempfile::tempdir()` for real filesystem integration tests rather than complex mock objects. This exercises actual OS file operations, permissions, and atomic renames, automatically cleaning up when the `TempDir` drops:

```rust
#[test]
fn fixes_file_in_place_atomically() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test.md");
    std::fs::write(&file_path, "# Title   \n").unwrap();

    let mut cmd = Command::cargo_bin("machdown").unwrap();
    cmd.arg("fix").arg(&file_path);
    cmd.assert().success().code(0);

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "# Title\n");
}
```
