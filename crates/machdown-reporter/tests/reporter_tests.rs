use machdown_reporter::format_diagnostic_plain;
use machdown_rules::{Diagnostic, Severity};
use std::path::PathBuf;

#[test]
fn test_format_diagnostic_plain() {
    let diag = Diagnostic::new(
        "MD009",
        "no-trailing-spaces",
        "Trailing whitespace".to_string(),
        2,
        16,
        Severity::Warning,
        None,
    )
    .with_file_path(PathBuf::from("doc.md"));

    let formatted = format_diagnostic_plain(&diag);
    assert_eq!(
        formatted,
        "doc.md:2:16: warning: [MD009] Trailing whitespace"
    );
}
