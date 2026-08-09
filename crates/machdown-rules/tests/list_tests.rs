use machdown_parser::parse;
use machdown_rules::rules::MD004UnorderedListStyle;
use machdown_rules::Rule;

#[test]
fn test_md004_unordered_list_style() {
    let rule = MD004UnorderedListStyle::default();
    let doc = parse("* Item 1\n- Item 2 Inconsistent\n+ Item 3 Inconsistent\n");
    let diagnostics = rule.check(&doc);

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].rule_id(), "MD004");
    assert_eq!(diagnostics[0].line(), 2);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(diagnostics[0].fix().unwrap().replacement(), "*");

    assert_eq!(diagnostics[1].rule_id(), "MD004");
    assert_eq!(diagnostics[1].line(), 3);
    assert!(diagnostics[1].is_fixable());
    assert_eq!(diagnostics[1].fix().unwrap().replacement(), "*");
}

#[test]
fn test_md005_list_indent() {
    use machdown_rules::rules::MD005ListIndent;
    let rule = MD005ListIndent;
    let doc = parse("* Level 0 item 1\n * Level 0 item 2 inconsistent\n");
    let diagnostics = rule.check(&doc);

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD005");
    assert_eq!(diagnostics[0].line(), 2);
}

#[test]
fn test_md007_ul_indent() {
    use machdown_rules::rules::MD007UlIndent;
    let rule = MD007UlIndent::new(2);
    let doc = parse("* Level 0\n   * Level 1 with 3 spaces\n");
    let diagnostics = rule.check(&doc);

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD007");
    assert_eq!(diagnostics[0].line(), 2);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(diagnostics[0].fix().unwrap().replacement(), "  ");
}

#[test]
fn test_md029_ol_prefix() {
    use machdown_rules::rules::{MD029OlPrefix, OlPrefixStyle};

    let rule = MD029OlPrefix::new(OlPrefixStyle::Ordered);
    let doc = parse("1. Item 1\n1. Item 2\n1. Item 3\n");
    let diagnostics = rule.check(&doc);

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].rule_id(), "MD029");
    assert_eq!(diagnostics[0].line(), 2);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(diagnostics[0].fix().unwrap().replacement(), "2.");

    assert_eq!(diagnostics[1].rule_id(), "MD029");
    assert_eq!(diagnostics[1].line(), 3);
    assert!(diagnostics[1].is_fixable());
    assert_eq!(diagnostics[1].fix().unwrap().replacement(), "3.");
}

#[test]
fn test_md030_spaces_after_list_marker() {
    use machdown_rules::rules::MD030SpacesAfterListMarker;

    let rule = MD030SpacesAfterListMarker::default();
    let doc = parse("*   Multiple spaces after marker\n");
    let diagnostics = rule.check(&doc);

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD030");
    assert_eq!(diagnostics[0].line(), 1);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(diagnostics[0].fix().unwrap().replacement(), " ");
}

#[test]
fn test_md032_blanks_around_lists() {
    use machdown_rules::rules::MD032BlanksAroundLists;

    let rule = MD032BlanksAroundLists;
    let doc = parse("Text before\n* List item 1\n* List item 2\nText after\n");
    let diagnostics = rule.check(&doc);

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].rule_id(), "MD032");
    assert_eq!(diagnostics[0].line(), 2);
    assert!(diagnostics[0].is_fixable());

    assert_eq!(diagnostics[1].rule_id(), "MD032");
    assert_eq!(diagnostics[1].line(), 3);
    assert!(diagnostics[1].is_fixable());
}

#[test]
fn test_md004_unordered_list_style_options() {
    use machdown_rules::rules::UnorderedListStyle;

    let rule = MD004UnorderedListStyle::new(UnorderedListStyle::Dash);
    let doc = parse("* Item 1\n+ Item 2\n");
    let diagnostics = rule.check(&doc);

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].fix().unwrap().replacement(), "-");
    assert_eq!(diagnostics[1].fix().unwrap().replacement(), "-");
}

#[test]
fn test_md029_ol_prefix_options() {
    use machdown_rules::rules::{MD029OlPrefix, OlPrefixStyle};

    let rule_one = MD029OlPrefix::new(OlPrefixStyle::One);
    let doc_ordered = parse("1. Item 1\n2. Item 2\n3. Item 3\n");
    let diagnostics_one = rule_one.check(&doc_ordered);
    assert_eq!(diagnostics_one.len(), 2);
    assert_eq!(diagnostics_one[0].fix().unwrap().replacement(), "1.");
    assert_eq!(diagnostics_one[1].fix().unwrap().replacement(), "1.");

    let rule_zero = MD029OlPrefix::new(OlPrefixStyle::Zero);
    let doc_one = parse("1. Item 1\n1. Item 2\n");
    let diagnostics_zero = rule_zero.check(&doc_one);
    assert_eq!(diagnostics_zero.len(), 2);
    assert_eq!(diagnostics_zero[0].fix().unwrap().replacement(), "0.");
    assert_eq!(diagnostics_zero[1].fix().unwrap().replacement(), "0.");
}

#[test]
fn test_md032_blanks_around_lists_clean() {
    use machdown_rules::rules::MD032BlanksAroundLists;

    let rule = MD032BlanksAroundLists;
    let doc = parse("\nText before\n\n* List item 1\n* List item 2\n\nText after\n");
    let diagnostics = rule.check(&doc);

    assert_eq!(diagnostics.len(), 0);
}
