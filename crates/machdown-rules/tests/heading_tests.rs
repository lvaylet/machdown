use machdown_parser::parse;
use machdown_rules::rules::{
    MD001HeadingIncrement, MD002FirstHeadingLevel, MD018NoMissingSpaceAtx, MD019NoMultipleSpaceAtx,
    MD020NoMissingSpaceClosedAtx, MD021NoMultipleSpaceClosedAtx, MD023HeadingStartLeft,
    MD025MultipleTopLevelHeadings,
};
use machdown_rules::Rule;

#[test]
fn test_md001_heading_increment() {
    let rule = MD001HeadingIncrement;
    let doc = parse("# H1\n### H3 Jumps level\n## H2\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD001");
    assert_eq!(diagnostics[0].line(), 2);
}

#[test]
fn test_md002_first_heading_level() {
    let rule = MD002FirstHeadingLevel::default();
    let doc = parse("## H2 Starts Document\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD002");
    assert_eq!(diagnostics[0].line(), 1);
}

#[test]
fn test_md025_multiple_top_level_headings() {
    let rule = MD025MultipleTopLevelHeadings::default();
    let doc = parse("# First H1\n## H2\n# Second H1\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD025");
    assert_eq!(diagnostics[0].line(), 3);
}

#[test]
fn test_md018_no_missing_space_atx() {
    let rule = MD018NoMissingSpaceAtx;
    let doc = parse("#Heading Without Space\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD018");
    assert_eq!(diagnostics[0].line(), 1);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(
        diagnostics[0].fix().unwrap().replacement(),
        "# Heading Without Space\n"
    );
}

#[test]
fn test_md019_no_multiple_space_atx() {
    let rule = MD019NoMultipleSpaceAtx;
    let doc = parse("#   Heading With Extra Spaces\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD019");
    assert_eq!(diagnostics[0].line(), 1);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(
        diagnostics[0].fix().unwrap().replacement(),
        "# Heading With Extra Spaces\n"
    );
}

#[test]
fn test_md020_no_missing_space_closed_atx() {
    let rule = MD020NoMissingSpaceClosedAtx;
    let doc = parse("#Closed Heading#\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD020");
    assert_eq!(diagnostics[0].line(), 1);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(
        diagnostics[0].fix().unwrap().replacement(),
        "# Closed Heading #\n"
    );
}

#[test]
fn test_md021_no_multiple_space_closed_atx() {
    let rule = MD021NoMultipleSpaceClosedAtx;
    let doc = parse("#   Closed Heading   #\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD021");
    assert_eq!(diagnostics[0].line(), 1);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(
        diagnostics[0].fix().unwrap().replacement(),
        "# Closed Heading #\n"
    );
}

#[test]
fn test_md023_heading_start_left() {
    let rule = MD023HeadingStartLeft;
    let doc = parse("  # Indented Heading\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD023");
    assert_eq!(diagnostics[0].line(), 1);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(
        diagnostics[0].fix().unwrap().replacement(),
        "# Indented Heading\n"
    );
}

#[test]
fn test_md003_heading_style_consistent_and_conversion() {
    use machdown_rules::rules::md003_heading_style::{HeadingStyleOption, MD003HeadingStyle};

    let rule = MD003HeadingStyle::new(HeadingStyleOption::Atx);
    let doc = parse("Title Level 1\n===\n# Closed Title #\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].rule_id(), "MD003");
    assert_eq!(diagnostics[0].line(), 1);
    assert!(diagnostics[0].is_fixable());
    assert_eq!(
        diagnostics[0].fix().unwrap().replacement(),
        "# Title Level 1\n"
    );

    assert_eq!(diagnostics[1].rule_id(), "MD003");
    assert_eq!(diagnostics[1].line(), 3);
    assert!(diagnostics[1].is_fixable());
    assert_eq!(
        diagnostics[1].fix().unwrap().replacement(),
        "# Closed Title\n"
    );
}

#[test]
fn test_md022_blanks_around_headings() {
    use machdown_rules::rules::md022_blanks_around_headings::MD022BlanksAroundHeadings;

    let rule = MD022BlanksAroundHeadings::default();
    let doc = parse("# H1 Title\nSome text immediately after\n\n## H2 Title\n\nSome text\n");
    let diagnostics = rule.check(&doc);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].rule_id(), "MD022");
    assert_eq!(diagnostics[0].line(), 1);
    assert!(diagnostics[0].is_fixable());
}
