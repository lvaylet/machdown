use machdown_parser::Document;

use crate::{Diagnostic, Rule, Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MD005ListIndent;

impl Rule for MD005ListIndent {
    fn id(&self) -> &'static str {
        "MD005"
    }

    fn name(&self) -> &'static str {
        "list-indent"
    }

    fn description(&self) -> &'static str {
        "Inconsistent indentation for list items at the same level"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for list in doc.lists() {
            if list.items.is_empty() {
                continue;
            }
            let mut level_indents = vec![list.items[0].indentation];

            for item in &list.items {
                let curr_indent = item.indentation;
                let last_indent = *level_indents.last().unwrap();

                if curr_indent > last_indent {
                    if curr_indent - last_indent < 2 {
                        diagnostics.push(Diagnostic::new(
                            self.id(),
                            self.name(),
                            format!(
                                "Inconsistent indentation for list item at same level (expected {}, found {})",
                                last_indent, curr_indent
                            ),
                            item.line_number,
                            curr_indent + 1,
                            Severity::Warning,
                            None,
                        ));
                    } else {
                        level_indents.push(curr_indent);
                    }
                } else if curr_indent < last_indent {
                    while level_indents.len() > 1 && *level_indents.last().unwrap() > curr_indent {
                        level_indents.pop();
                    }
                    let matched_indent = *level_indents.last().unwrap();
                    if curr_indent != matched_indent {
                        diagnostics.push(Diagnostic::new(
                            self.id(),
                            self.name(),
                            format!(
                                "Inconsistent indentation for list item at same level (expected {}, found {})",
                                matched_indent, curr_indent
                            ),
                            item.line_number,
                            curr_indent + 1,
                            Severity::Warning,
                            None,
                        ));
                    }
                }
            }
        }
        diagnostics
    }
}
