use crate::{Diagnostic, Rule, Severity};
use machdown_parser::Document;

#[derive(Debug, Clone)]
pub struct MD002FirstHeadingLevel {
    pub level: u8,
}

impl Default for MD002FirstHeadingLevel {
    fn default() -> Self {
        Self { level: 1 }
    }
}

impl Rule for MD002FirstHeadingLevel {
    fn id(&self) -> &'static str {
        "MD002"
    }

    fn name(&self) -> &'static str {
        "first-heading-h1"
    }

    fn description(&self) -> &'static str {
        "First heading should be a top level heading"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let headings = doc.headings();

        if let Some(first) = headings.first() {
            if first.level != self.level {
                diagnostics.push(Diagnostic::new(
                    self.id(),
                    self.name(),
                    format!(
                        "First heading should be a level {} heading [Actual: {}]",
                        self.level, first.level
                    ),
                    first.span.start_line,
                    first.span.start_col,
                    Severity::Warning,
                    None,
                ));
            }
        }

        diagnostics
    }
}
