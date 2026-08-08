use machdown_parser::Document;

use crate::{Diagnostic, Edit, Rule, Severity, Span};

/// MD009: Trailing whitespace.
#[derive(Debug, Default, Clone)]
pub struct MD009TrailingWhitespace;

impl Rule for MD009TrailingWhitespace {
    fn id(&self) -> &'static str {
        "MD009"
    }

    fn name(&self) -> &'static str {
        "no-trailing-spaces"
    }

    fn description(&self) -> &'static str {
        "Trailing whitespace"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for line in doc.lines() {
            let content = line.content();
            let trimmed = content.trim_end_matches([' ', '\t']);
            if trimmed.len() < content.len() {
                let trailing_len = content.len() - trimmed.len();
                let col = trimmed.chars().count() + 1;
                let start_byte = line.byte_offset() + trimmed.len();
                let end_byte = start_byte + trailing_len;

                let span = Span::new(line.line_number(), col, start_byte, end_byte);
                let edit = Edit::new(span, String::new());

                diagnostics.push(Diagnostic::new(
                    self.id(),
                    self.name(),
                    "Trailing whitespace".to_string(),
                    line.line_number(),
                    col,
                    Severity::Warning,
                    Some(edit),
                ));
            }
        }

        diagnostics
    }
}
