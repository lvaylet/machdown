use machdown_fixer::{fix_file_in_place, fix_str};
use machdown_rules::default_rules;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_fix_str_removes_trailing_whitespace() {
    let source = "Line 1   \nLine 2\t\nLine 3";
    let rules = default_rules();

    let (fixed, count) = fix_str(source, &rules);

    assert_eq!(fixed, "Line 1\nLine 2\nLine 3");
    assert_eq!(count, 2);
}

#[test]
fn test_fix_str_is_idempotent() {
    let source = "Line 1   \nLine 2\t\nLine 3";
    let rules = default_rules();

    let (fixed_pass1, count1) = fix_str(source, &rules);
    assert_eq!(count1, 2);

    let (fixed_pass2, count2) = fix_str(&fixed_pass1, &rules);
    assert_eq!(count2, 0);
    assert_eq!(fixed_pass1, fixed_pass2);
}

#[test]
fn test_fix_file_in_place_atomic() {
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "First line  \nSecond line\n").unwrap();
    temp_file.flush().unwrap();

    let path = temp_file.path().to_path_buf();
    let rules = default_rules();

    let count = fix_file_in_place(&path, &rules).unwrap();
    assert_eq!(count, 1);

    let content_after = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content_after, "First line\nSecond line\n");
}
