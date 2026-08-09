use crate::{Diagnostic, Rule, Severity};
use machdown_parser::Document;

pub struct MD001HeadingIncrement;

impl Rule for MD001HeadingIncrement {
    fn id(&self) -> &'static str {
        "MD001"
    }

    fn name(&self) -> &'static str {
        "heading-increment"
    }

    fn description(&self) -> &'static str {
        "Heading levels should only increment by one level at a time"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let headings = doc.headings();
        let mut prev_level = 0;

        for heading in headings {
            if prev_level > 0 && heading.level > prev_level + 1 {
                diagnostics.push(Diagnostic::new(
                    self.id(),
                    self.name(),
                    format!(
                        "Heading levels should only increment by one level at a time [Expected: <= {}, Actual: {}]",
                        prev_level + 1,
                        heading.level
                    ),
                    heading.span.start_line,
                    heading.span.start_col,
                    Severity::Warning,
                    None,
                ));
            }
            prev_level = heading.level;
        }

        diagnostics
    }
}
