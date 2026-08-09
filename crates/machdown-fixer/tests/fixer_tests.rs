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

#[test]
fn test_heading_rules_autofix_and_idempotency_golden() {
    let dirty_input = "\
#Heading Without Space
Some text immediately after
  ##   Indented Multiple Spaces Title   
Another line of body text
###Closed ATX Heading###
Final text
";

    let rules = default_rules();
    let (fixed, count) = fix_str(dirty_input, &rules);

    assert!(count > 0);
    let expected_fixed = "\
# Heading Without Space

Some text immediately after

## Indented Multiple Spaces Title

Another line of body text

### Closed ATX Heading

Final text
";
    assert_eq!(fixed, expected_fixed);

    // Assert idempotency: second pass produces 0 fixes
    let (fixed_pass2, count2) = fix_str(&fixed, &rules);
    assert_eq!(count2, 0);
    assert_eq!(fixed_pass2, expected_fixed);
}

#[test]
fn test_obsidian_syntax_in_headings_preserved_during_fixes() {
    let obsidian_input = "\
---
title: Note Title
tags: [test, obsidian]
---
#   Heading With [[WikiLink Target|Custom Alias]]   
Body text mentioning $E=mc^2$ equation.

  ## Heading With Math $a^2 + b^2 = c^2$ and #project/alpha   
More body text.
";

    let rules = default_rules();
    let (fixed, count) = fix_str(obsidian_input, &rules);

    assert!(count > 0);
    assert!(fixed.contains("# Heading With [[WikiLink Target|Custom Alias]]"));
    assert!(fixed.contains("## Heading With Math $a^2 + b^2 = c^2$ and #project/alpha"));
    assert!(fixed.contains("---"));
    assert!(fixed.contains("tags: [test, obsidian]"));

    // Idempotency
    let (fixed_pass2, count2) = fix_str(&fixed, &rules);
    assert_eq!(count2, 0);
    assert_eq!(fixed_pass2, fixed);
}
