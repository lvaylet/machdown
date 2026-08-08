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
