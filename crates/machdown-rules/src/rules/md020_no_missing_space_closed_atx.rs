use crate::{Diagnostic, Edit, Rule, Severity, Span};
use machdown_parser::{Document, HeadingStyle};

pub struct MD020NoMissingSpaceClosedAtx;

impl Rule for MD020NoMissingSpaceClosedAtx {
    fn id(&self) -> &'static str {
        "MD020"
    }

    fn name(&self) -> &'static str {
        "no-missing-space-closed-atx"
    }

    fn description(&self) -> &'static str {
        "No space inside hashes on closed ATX style heading"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for h in doc.headings() {
            if h.style == HeadingStyle::AtxClosed
                && (h.opening_spaces == 0 || h.closing_spaces == 0)
            {
                let line_ending = doc
                    .lines()
                    .get(h.span.start_line - 1)
                    .map(|l| l.ending().as_str())
                    .unwrap_or("\n");

                let hashes = "#".repeat(h.level as usize);
                let closing_hashes = "#".repeat(h.closing_hash_count);
                let leading = " ".repeat(h.leading_spaces);
                let replacement = format!(
                    "{}{hashes} {} {closing_hashes}{line_ending}",
                    leading, h.title
                );

                let rule_span = Span::new(
                    h.span.start_line,
                    h.span.start_col,
                    h.span.start_byte,
                    h.span.end_byte,
                );

                diagnostics.push(Diagnostic::new(
                    self.id(),
                    self.name(),
                    "No space inside hashes on closed ATX style heading".to_string(),
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
