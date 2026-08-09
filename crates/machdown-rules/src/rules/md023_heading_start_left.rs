use crate::{Diagnostic, Edit, Rule, Severity, Span};
use machdown_parser::{Document, HeadingStyle};

pub struct MD023HeadingStartLeft;

impl Rule for MD023HeadingStartLeft {
    fn id(&self) -> &'static str {
        "MD023"
    }

    fn name(&self) -> &'static str {
        "heading-start-left"
    }

    fn description(&self) -> &'static str {
        "Heading must start at the beginning of the line"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for h in doc.headings() {
            if h.leading_spaces > 0 {
                let line_ending = doc
                    .lines()
                    .get(h.span.start_line - 1)
                    .map(|l| l.ending().as_str())
                    .unwrap_or("\n");

                let replacement = match h.style {
                    HeadingStyle::Atx => {
                        let hashes = "#".repeat(h.level as usize);
                        let spaces = " ".repeat(h.opening_spaces);
                        format!("{hashes}{spaces}{}{line_ending}", h.title)
                    }
                    HeadingStyle::AtxClosed => {
                        let hashes = "#".repeat(h.level as usize);
                        let op_spaces = " ".repeat(h.opening_spaces);
                        let cl_spaces = " ".repeat(h.closing_spaces);
                        let closing_hashes = "#".repeat(h.closing_hash_count);
                        format!(
                            "{hashes}{op_spaces}{}{cl_spaces}{closing_hashes}{line_ending}",
                            h.title
                        )
                    }
                    HeadingStyle::Setext => {
                        let underline_char = if h.level == 1 { '=' } else { '-' };
                        let underline = underline_char.to_string().repeat(h.title.len().max(3));
                        format!("{}{line_ending}{underline}{line_ending}", h.title)
                    }
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
                    "Heading must start at the beginning of the line".to_string(),
                    h.span.start_line,
                    1,
                    Severity::Warning,
                    Some(Edit::new(rule_span, replacement)),
                ));
            }
        }
        diagnostics
    }
}
