use machdown_parser::parse;
use machdown_rules::{MD009TrailingWhitespace, Rule};

#[test]
fn test_md009_detects_and_fixes_trailing_whitespace() {
    let source = "Clean line\nTrailing spaces  \nTrailing tab\t\n";
    let doc = parse(source);

    let rule = MD009TrailingWhitespace;
    let diagnostics = rule.check(&doc);

    assert_eq!(diagnostics.len(), 2);

    // First violation: line 2, column 16 (after "Trailing spaces")
    let d1 = &diagnostics[0];
    assert_eq!(d1.rule_id(), "MD009");
    assert_eq!(d1.line(), 2);
    assert_eq!(d1.column(), 16);
    assert!(d1.fix().is_some());
    let edit1 = d1.fix().unwrap();
    assert_eq!(edit1.replacement(), "");

    // Second violation: line 3, column 13 (after "Trailing tab")
    let d2 = &diagnostics[1];
    assert_eq!(d2.rule_id(), "MD009");
    assert_eq!(d2.line(), 3);
    assert_eq!(d2.column(), 13);
    assert!(d2.fix().is_some());
}

#[test]
fn test_md009_clean_line_has_no_diagnostics() {
    let source = "Clean line 1\nClean line 2\n";
    let doc = parse(source);

    let rule = MD009TrailingWhitespace;
    let diagnostics = rule.check(&doc);

    assert!(diagnostics.is_empty());
}
