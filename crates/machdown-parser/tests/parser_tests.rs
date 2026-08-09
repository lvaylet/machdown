use machdown_parser::{parse, LineEnding};

#[test]
fn test_parse_lines_preserves_byte_offsets_and_line_numbers() {
    let source = "First line\nSecond line  \nThird line";
    let doc = parse(source);

    assert_eq!(doc.lines().len(), 3);

    let line1 = &doc.lines()[0];
    assert_eq!(line1.line_number(), 1);
    assert_eq!(line1.content(), "First line");
    assert_eq!(line1.byte_offset(), 0);
    assert_eq!(line1.byte_len(), 11);
    assert_eq!(line1.ending(), LineEnding::Lf);

    let line2 = &doc.lines()[1];
    assert_eq!(line2.line_number(), 2);
    assert_eq!(line2.content(), "Second line  ");
    assert_eq!(line2.byte_offset(), 11);
    assert_eq!(line2.byte_len(), 14);
    assert_eq!(line2.ending(), LineEnding::Lf);

    let line3 = &doc.lines()[2];
    assert_eq!(line3.line_number(), 3);
    assert_eq!(line3.content(), "Third line");
    assert_eq!(line3.byte_offset(), 25);
    assert_eq!(line3.byte_len(), 10);
    assert_eq!(line3.ending(), LineEnding::None);
}

#[test]
fn test_parse_crlf_line_endings() {
    let source = "Line 1\r\nLine 2\r\n";
    let doc = parse(source);

    assert_eq!(doc.lines().len(), 2);

    assert_eq!(doc.lines()[0].content(), "Line 1");
    assert_eq!(doc.lines()[0].ending(), LineEnding::CrLf);
    assert_eq!(doc.lines()[0].byte_len(), 8);

    assert_eq!(doc.lines()[1].content(), "Line 2");
    assert_eq!(doc.lines()[1].ending(), LineEnding::CrLf);
    assert_eq!(doc.lines()[1].byte_len(), 8);
}

#[test]
fn test_parse_yaml_frontmatter() {
    let source = "---\ntitle: Note Title\ntags:\n  - project/alpha\n---\n# Heading\n";
    let doc = parse(source);

    let frontmatter = doc.frontmatter().expect("should parse frontmatter");
    assert_eq!(
        frontmatter.raw_content(),
        "title: Note Title\ntags:\n  - project/alpha"
    );
    assert_eq!(frontmatter.span().start_byte, 0);
    assert_eq!(frontmatter.span().end_byte, 50);
}

#[test]
fn test_parse_wiki_links() {
    let source = "See [[Note A]] and [[Note B|Display B]] here.\n";
    let doc = parse(source);

    let wiki_links = doc.wiki_links();
    assert_eq!(wiki_links.len(), 2);

    assert_eq!(wiki_links[0].target, "Note A");
    assert_eq!(wiki_links[0].alias, None);
    assert_eq!(wiki_links[0].span.start_byte, 4);
    assert_eq!(wiki_links[0].span.end_byte, 14); // `[[Note A]]` is 10 chars -> 4..14

    assert_eq!(wiki_links[1].target, "Note B");
    assert_eq!(wiki_links[1].alias.as_deref(), Some("Display B"));
    assert_eq!(wiki_links[1].span.start_byte, 19);
    assert_eq!(wiki_links[1].span.end_byte, 39); // `[[Note B|Display B]]` is 20 chars -> 19..39
}

#[test]
fn test_parse_callouts() {
    let source = "> [!NOTE] Custom Title\n> Callout body line 1\n> Callout body line 2 with [[Linked Note]]\n";
    let doc = parse(source);

    let callouts = doc.callouts();
    assert_eq!(callouts.len(), 1);

    let callout = callouts[0];
    assert_eq!(callout.kind, "NOTE");
    assert_eq!(callout.title.as_deref(), Some("Custom Title"));
    assert_eq!(callout.span.start_line, 1);
    assert_eq!(callout.span.end_line, 3);

    // Callout nested contents should also be parsed (e.g., wiki links inside callout)
    let inner_links = doc.wiki_links();
    assert_eq!(inner_links.len(), 1);
    assert_eq!(inner_links[0].target, "Linked Note");
}

#[test]
fn test_parse_block_identifiers() {
    let source = "Some important statement. ^block-id-123\n";
    let doc = parse(source);

    let block_ids = doc.block_ids();
    assert_eq!(block_ids.len(), 1);
    assert_eq!(block_ids[0].id, "block-id-123");
    assert_eq!(block_ids[0].span.start_byte, 26);
    assert_eq!(block_ids[0].span.end_byte, 39);
}

#[test]
fn test_parse_math() {
    let source = "Formula: $E=mc^2$ and block math:\n$$\n\\sum_{i=1}^n i\n$$\n";
    let doc = parse(source);

    let inline_math = doc.inline_math();
    assert_eq!(inline_math.len(), 1);
    assert_eq!(inline_math[0].math, "E=mc^2");

    let display_math = doc.display_math();
    assert_eq!(display_math.len(), 1);
    assert_eq!(display_math[0].math.trim(), "\\sum_{i=1}^n i");
}

#[test]
fn test_parse_nested_hashtags() {
    let source = "# Real Heading\nWorking on #project/alpha and #todo/urgent in prose.\n";
    let doc = parse(source);

    let tags = doc.tags();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].tag, "project/alpha");
    assert_eq!(tags[1].tag, "todo/urgent");
}

#[test]
fn test_obsidian_sample_note_roundtrip_fidelity() {
    let sample = "---
title: Comprehensive Obsidian Note
tags:
  - project/alpha
  - status/wip
---
# Main Section Title

This note has a wiki link to [[Project Overview|Overview]] and inline math $E=mc^2$.

> [!NOTE] Callout Title
> Here is callout body with #inline/tag and [[Nested Note]].

$$
\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}
$$

Paragraph with block ID anchor. ^block-anchor-99
";

    let doc = parse(sample);

    // Verify round-trip source reconstruction
    assert_eq!(doc.reconstruct_source(), sample);

    // Verify all Obsidian AST nodes recognized without corruption
    assert!(doc.frontmatter().is_some());
    assert_eq!(doc.frontmatter().unwrap().span().start_byte, 0);

    let links = doc.wiki_links();
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].target, "Project Overview");
    assert_eq!(links[0].alias.as_deref(), Some("Overview"));
    assert_eq!(links[1].target, "Nested Note");

    let callouts = doc.callouts();
    assert_eq!(callouts.len(), 1);
    assert_eq!(callouts[0].kind, "NOTE");

    let inline_m = doc.inline_math();
    assert_eq!(inline_m.len(), 1);
    assert_eq!(inline_m[0].math, "E=mc^2");

    let display_m = doc.display_math();
    assert_eq!(display_m.len(), 1);

    let block_ids = doc.block_ids();
    assert_eq!(block_ids.len(), 1);
    assert_eq!(block_ids[0].id, "block-anchor-99");
}

#[test]
fn test_parse_headings_atx_setext_closed() {
    use machdown_parser::HeadingStyle;

    let source = "# ATX Level 1\n##  ATX Multi Space\n#NoSpaceATX\n# Closed ATX #\n##  Closed Multi Space  ##\nSetext Level 1\n===\nSetext Level 2\n---\n";
    let doc = parse(source);
    let headings = doc.headings();
    assert_eq!(headings.len(), 7);

    assert_eq!(headings[0].level, 1);
    assert_eq!(headings[0].style, HeadingStyle::Atx);
    assert_eq!(headings[0].title, "ATX Level 1");
    assert_eq!(headings[0].opening_spaces, 1);
    assert_eq!(headings[0].leading_spaces, 0);

    assert_eq!(headings[1].level, 2);
    assert_eq!(headings[1].style, HeadingStyle::Atx);
    assert_eq!(headings[1].title, "ATX Multi Space");
    assert_eq!(headings[1].opening_spaces, 2);

    assert_eq!(headings[2].level, 1);
    assert_eq!(headings[2].style, HeadingStyle::Atx);
    assert_eq!(headings[2].title, "NoSpaceATX");
    assert_eq!(headings[2].opening_spaces, 0);

    assert_eq!(headings[3].level, 1);
    assert_eq!(headings[3].style, HeadingStyle::AtxClosed);
    assert_eq!(headings[3].title, "Closed ATX");
    assert_eq!(headings[3].opening_spaces, 1);
    assert_eq!(headings[3].closing_spaces, 1);

    assert_eq!(headings[4].level, 2);
    assert_eq!(headings[4].style, HeadingStyle::AtxClosed);
    assert_eq!(headings[4].title, "Closed Multi Space");
    assert_eq!(headings[4].opening_spaces, 2);
    assert_eq!(headings[4].closing_spaces, 2);

    assert_eq!(headings[5].level, 1);
    assert_eq!(headings[5].style, HeadingStyle::Setext);
    assert_eq!(headings[5].title, "Setext Level 1");

    assert_eq!(headings[6].level, 2);
    assert_eq!(headings[6].style, HeadingStyle::Setext);
    assert_eq!(headings[6].title, "Setext Level 2");
}

#[test]
fn test_parse_lists() {
    let source = "* Item 1\n* Item 2\n  1. Subitem 1\n  2. Subitem 2\n";
    let doc = parse(source);

    let list_items = doc.list_items();
    assert_eq!(list_items.len(), 4);
    assert!(!list_items[0].is_ordered);
    assert_eq!(list_items[0].marker, "*");
    assert_eq!(list_items[0].indentation, 0);
    assert_eq!(list_items[0].spaces_after_marker, 1);

    assert!(list_items[2].is_ordered);
    assert_eq!(list_items[2].marker, "1.");
    assert_eq!(list_items[2].indentation, 2);
    assert_eq!(list_items[2].spaces_after_marker, 1);

    let lists = doc.lists();
    assert_eq!(lists.len(), 2);
    assert_eq!(lists[0].items.len(), 2);
    assert_eq!(lists[1].items.len(), 2);
}
