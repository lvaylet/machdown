use crate::{Diagnostic, Rule, Severity};
use machdown_parser::Document;

#[derive(Debug, Clone)]
pub struct MD025MultipleTopLevelHeadings {
    pub level: u8,
}

impl Default for MD025MultipleTopLevelHeadings {
    fn default() -> Self {
        Self { level: 1 }
    }
}

impl Rule for MD025MultipleTopLevelHeadings {
    fn id(&self) -> &'static str {
        "MD025"
    }

    fn name(&self) -> &'static str {
        "single-title"
    }

    fn description(&self) -> &'static str {
        "Multiple top-level headings in the same document"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let headings = doc.headings();

        let top_level_headings: Vec<_> = headings
            .into_iter()
            .filter(|h| h.level == self.level)
            .collect();

        if top_level_headings.len() > 1 {
            for heading in top_level_headings.into_iter().skip(1) {
                diagnostics.push(Diagnostic::new(
                    self.id(),
                    self.name(),
                    format!(
                        "Multiple top-level headings in the same document [Expected: 1, Actual: level {}]",
                        self.level
                    ),
                    heading.span.start_line,
                    heading.span.start_col,
                    Severity::Warning,
                    None,
                ));
            }
        }

        diagnostics
    }
}
