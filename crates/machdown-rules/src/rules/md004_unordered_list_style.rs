use machdown_parser::Document;

use crate::{Diagnostic, Edit, Rule, Severity, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnorderedListStyle {
    #[default]
    Consistent,
    Asterisk,
    Dash,
    Plus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MD004UnorderedListStyle {
    pub style: UnorderedListStyle,
}

impl MD004UnorderedListStyle {
    pub fn new(style: UnorderedListStyle) -> Self {
        Self { style }
    }
}

impl Rule for MD004UnorderedListStyle {
    fn id(&self) -> &'static str {
        "MD004"
    }

    fn name(&self) -> &'static str {
        "ul-style"
    }

    fn description(&self) -> &'static str {
        "Unordered list bullet style consistency"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let items = doc.list_items();
        let unordered_items: Vec<_> = items.into_iter().filter(|it| !it.is_ordered).collect();
        if unordered_items.is_empty() {
            return diagnostics;
        }

        let target_bullet = match self.style {
            UnorderedListStyle::Asterisk => '*',
            UnorderedListStyle::Dash => '-',
            UnorderedListStyle::Plus => '+',
            UnorderedListStyle::Consistent => unordered_items
                .first()
                .and_then(|it| it.bullet_char)
                .unwrap_or('*'),
        };

        for item in unordered_items {
            if let Some(bullet) = item.bullet_char {
                if bullet != target_bullet {
                    let edit = Edit::new(
                        Span::new(
                            item.line_number,
                            item.marker_span.start_col,
                            item.marker_span.start_byte,
                            item.marker_span.end_byte,
                        ),
                        target_bullet.to_string(),
                    );

                    diagnostics.push(Diagnostic::new(
                        self.id(),
                        self.name(),
                        format!(
                            "Unordered list item bullet style inconsistent with expected '{}' (found '{}')",
                            target_bullet, bullet
                        ),
                        item.line_number,
                        item.marker_span.start_col,
                        Severity::Warning,
                        Some(edit),
                    ));
                }
            }
        }

        diagnostics
    }
}
