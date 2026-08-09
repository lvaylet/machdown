use machdown_parser::Document;

use crate::{Diagnostic, Edit, Rule, Severity, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MD007UlIndent {
    pub indent: usize,
}

impl Default for MD007UlIndent {
    fn default() -> Self {
        Self { indent: 2 }
    }
}

impl MD007UlIndent {
    pub fn new(indent: usize) -> Self {
        Self { indent }
    }
}

impl Rule for MD007UlIndent {
    fn id(&self) -> &'static str {
        "MD007"
    }

    fn name(&self) -> &'static str {
        "ul-indent"
    }

    fn description(&self) -> &'static str {
        "Unordered list indentation spaces"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for list in doc.lists() {
            let ul_items: Vec<_> = list.items.iter().filter(|it| !it.is_ordered).collect();
            if ul_items.is_empty() {
                continue;
            }

            let mut level_indents = vec![ul_items[0].indentation];

            for item in &ul_items {
                let curr_indent = item.indentation;
                let last_indent = *level_indents.last().unwrap();

                let (level, expected_indent) = if curr_indent > last_indent {
                    level_indents.push(curr_indent);
                    let level = level_indents.len() - 1;
                    (level, level_indents[0] + level * self.indent)
                } else if curr_indent < last_indent {
                    while level_indents.len() > 1 && *level_indents.last().unwrap() > curr_indent {
                        level_indents.pop();
                    }
                    let level = level_indents.len() - 1;
                    (level, level_indents[0] + level * self.indent)
                } else {
                    let level = level_indents.len() - 1;
                    (level, level_indents[0] + level * self.indent)
                };

                if curr_indent != expected_indent {
                    let line_start_byte = item.span.start_byte;
                    let marker_start_byte = item.marker_span.start_byte;

                    let edit = Edit::new(
                        Span::new(item.line_number, 1, line_start_byte, marker_start_byte),
                        " ".repeat(expected_indent),
                    );

                    diagnostics.push(Diagnostic::new(
                        self.id(),
                        self.name(),
                        format!(
                            "Unordered list indentation at level {} expected {} spaces, found {}",
                            level, expected_indent, curr_indent
                        ),
                        item.line_number,
                        1,
                        Severity::Warning,
                        Some(edit),
                    ));
                }
            }
        }

        diagnostics
    }
}
