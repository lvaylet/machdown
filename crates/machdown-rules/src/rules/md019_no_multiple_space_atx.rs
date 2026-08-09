use crate::{Diagnostic, Edit, Rule, Severity, Span};
use machdown_parser::{Document, HeadingStyle};

pub struct MD019NoMultipleSpaceAtx;

impl Rule for MD019NoMultipleSpaceAtx {
    fn id(&self) -> &'static str {
        "MD019"
    }

    fn name(&self) -> &'static str {
        "no-multiple-space-atx"
    }

    fn description(&self) -> &'static str {
        "Multiple spaces after hash on ATX style heading"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for h in doc.headings() {
            if (h.style == HeadingStyle::Atx || h.style == HeadingStyle::AtxClosed)
                && h.opening_spaces > 1
            {
                let line_ending = doc
                    .lines()
                    .get(h.span.start_line - 1)
                    .map(|l| l.ending().as_str())
                    .unwrap_or("\n");

                let hashes = "#".repeat(h.level as usize);
                let leading = " ".repeat(h.leading_spaces);
                let replacement = if h.style == HeadingStyle::AtxClosed {
                    let closing_hashes = "#".repeat(h.closing_hash_count);
                    let c_spaces = " ".repeat(h.closing_spaces);
                    format!(
                        "{}{hashes} {}{c_spaces}{closing_hashes}{line_ending}",
                        leading, h.title
                    )
                } else {
                    format!("{}{hashes} {}{line_ending}", leading, h.title)
                };

                let rule_span = Span::new(
                    h.span.start_line,
                    h.span.start_col,
                    h.span.start_byte,
                    h.span.end_byte,
                );

                diagnostics.push(Diagnostic::new(
                    self.id(),
                    self.name(),
                    "Multiple spaces after hash on ATX style heading".to_string(),
                    h.span.start_line,
                    h.span.start_col,
                    Severity::Warning,
                    Some(Edit::new(rule_span, replacement)),
                ));
            }
        }
        diagnostics
    }
}
