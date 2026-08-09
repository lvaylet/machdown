use crate::{Diagnostic, Edit, Rule, Severity, Span};
use machdown_parser::{Document, HeadingStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadingStyleOption {
    Consistent,
    Atx,
    AtxClosed,
    Setext,
}

#[derive(Debug, Clone)]
pub struct MD003HeadingStyle {
    pub style: HeadingStyleOption,
}

impl MD003HeadingStyle {
    pub fn new(style: HeadingStyleOption) -> Self {
        Self { style }
    }
}

impl Default for MD003HeadingStyle {
    fn default() -> Self {
        Self {
            style: HeadingStyleOption::Consistent,
        }
    }
}

impl Rule for MD003HeadingStyle {
    fn id(&self) -> &'static str {
        "MD003"
    }

    fn name(&self) -> &'static str {
        "heading-style"
    }

    fn description(&self) -> &'static str {
        "Heading style consistency"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let headings = doc.headings();
        if headings.is_empty() {
            return diagnostics;
        }

        let target_style = match self.style {
            HeadingStyleOption::Consistent => match headings[0].style {
                HeadingStyle::Atx => HeadingStyleOption::Atx,
                HeadingStyle::AtxClosed => HeadingStyleOption::AtxClosed,
                HeadingStyle::Setext => HeadingStyleOption::Setext,
            },
            other => other,
        };

        for h in headings {
            let is_violation = match target_style {
                HeadingStyleOption::Atx => h.style != HeadingStyle::Atx,
                HeadingStyleOption::AtxClosed => h.style != HeadingStyle::AtxClosed,
                HeadingStyleOption::Setext => {
                    if h.level <= 2 {
                        h.style != HeadingStyle::Setext
                    } else {
                        false
                    }
                }
                HeadingStyleOption::Consistent => unreachable!(),
            };

            if is_violation {
                let line_ending = doc
                    .lines()
                    .get(h.span.start_line - 1)
                    .map(|l| l.ending().as_str())
                    .unwrap_or("\n");

                let replacement = match target_style {
                    HeadingStyleOption::Atx => {
                        let hashes = "#".repeat(h.level as usize);
                        format!("{hashes} {}{line_ending}", h.title)
                    }
                    HeadingStyleOption::AtxClosed => {
                        let hashes = "#".repeat(h.level as usize);
                        format!("{hashes} {} {hashes}{line_ending}", h.title)
                    }
                    HeadingStyleOption::Setext => {
                        let underline_char = if h.level == 1 { '=' } else { '-' };
                        let underline = underline_char.to_string().repeat(h.title.len().max(3));
                        format!("{}{line_ending}{underline}{line_ending}", h.title)
                    }
                    HeadingStyleOption::Consistent => unreachable!(),
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
                    format!(
                        "Heading style should be {:?} [Actual: {:?}]",
                        target_style, h.style
                    ),
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
