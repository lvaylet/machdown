use crate::{Diagnostic, Edit, Rule, Severity, Span};
use machdown_parser::Document;

#[derive(Debug, Clone)]
pub struct MD022BlanksAroundHeadings {
    pub lines_above: usize,
    pub lines_below: usize,
}

impl Default for MD022BlanksAroundHeadings {
    fn default() -> Self {
        Self {
            lines_above: 1,
            lines_below: 1,
        }
    }
}

impl Rule for MD022BlanksAroundHeadings {
    fn id(&self) -> &'static str {
        "MD022"
    }

    fn name(&self) -> &'static str {
        "blanks-around-headings"
    }

    fn description(&self) -> &'static str {
        "Headings should be surrounded by blank lines"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines = doc.lines();
        let total_lines = lines.len();

        let frontmatter_end_line = doc.frontmatter().map(|fm| fm.span.end_line).unwrap_or(0);

        for h in doc.headings() {
            let start_line = h.span.start_line;
            let end_line = h.span.end_line;

            // Check lines above
            let is_at_doc_start = start_line == 1;
            let is_after_frontmatter =
                frontmatter_end_line > 0 && start_line == frontmatter_end_line + 1;
            let need_above_check =
                !is_at_doc_start && !is_after_frontmatter && self.lines_above > 0;

            let mut missing_above = 0;
            if need_above_check {
                for k in 1..=self.lines_above {
                    if start_line > k {
                        let prev_idx = start_line - 1 - k;
                        if !lines[prev_idx].content().trim().is_empty() {
                            missing_above += 1;
                        }
                    }
                }
            }

            // Check lines below
            let is_at_doc_end = end_line == total_lines;
            let need_below_check = !is_at_doc_end && self.lines_below > 0;

            let mut missing_below = 0;
            if need_below_check {
                for k in 0..self.lines_below {
                    let next_idx = end_line + k;
                    if next_idx < total_lines && !lines[next_idx].content().trim().is_empty() {
                        missing_below += 1;
                    }
                }
            }

            if missing_above > 0 || missing_below > 0 {
                let line_ending = lines
                    .get(start_line - 1)
                    .map(|l| l.ending().as_str())
                    .unwrap_or("\n");

                let raw_heading = doc.source()[h.span.start_byte..h.span.end_byte].to_string();
                let above_padding = line_ending.repeat(missing_above);
                let below_padding = line_ending.repeat(missing_below);
                let replacement = format!("{above_padding}{raw_heading}{below_padding}");

                let rule_span = Span::new(
                    start_line,
                    h.span.start_col,
                    h.span.start_byte,
                    h.span.end_byte,
                );

                diagnostics.push(Diagnostic::new(
                    self.id(),
                    self.name(),
                    "Headings should be surrounded by blank lines".to_string(),
                    start_line,
                    h.span.start_col,
                    Severity::Warning,
                    Some(Edit::new(rule_span, replacement)),
                ));
            }
        }

        diagnostics
    }
}
