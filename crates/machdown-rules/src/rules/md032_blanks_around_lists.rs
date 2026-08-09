use machdown_parser::Document;

use crate::{Diagnostic, Edit, Rule, Severity, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MD032BlanksAroundLists;

impl Rule for MD032BlanksAroundLists {
    fn id(&self) -> &'static str {
        "MD032"
    }

    fn name(&self) -> &'static str {
        "blanks-around-lists"
    }

    fn description(&self) -> &'static str {
        "Lists should be surrounded by blank lines"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines = doc.lines();
        if lines.is_empty() {
            return diagnostics;
        }

        for list in doc.lists() {
            if list.items.is_empty() {
                continue;
            }

            let first_item = list.items.first().unwrap();
            let last_item = list.items.last().unwrap();

            let start_line_idx = first_item.line_number - 1;
            let end_line_idx = last_item.line_number - 1;

            // Check line above start of list
            if start_line_idx > 0 {
                let prev_line = &lines[start_line_idx - 1];
                if !prev_line.content().trim().is_empty() {
                    let ending = prev_line.ending().as_str();
                    let edit_ending = if ending.is_empty() { "\n" } else { ending };

                    let cur_line = &lines[start_line_idx];
                    let is_prev_list_item =
                        machdown_parser::try_parse_list_item(prev_line).is_some();
                    let edit = if is_prev_list_item {
                        None
                    } else {
                        Some(Edit::new(
                            Span::new(
                                cur_line.line_number(),
                                1,
                                cur_line.byte_offset(),
                                cur_line.byte_offset(),
                            ),
                            edit_ending.to_string(),
                        ))
                    };

                    diagnostics.push(Diagnostic::new(
                        self.id(),
                        self.name(),
                        "Lists should be preceded by a blank line".to_string(),
                        first_item.line_number,
                        1,
                        Severity::Warning,
                        edit,
                    ));
                }
            }

            // Check line below end of list
            if end_line_idx + 1 < lines.len() {
                let next_line = &lines[end_line_idx + 1];
                if !next_line.content().trim().is_empty() {
                    let ending = lines[end_line_idx].ending().as_str();
                    let edit_ending = if ending.is_empty() { "\n" } else { ending };

                    let insert_pos = next_line.byte_offset();
                    let edit = Edit::new(
                        Span::new(next_line.line_number(), 1, insert_pos, insert_pos),
                        edit_ending.to_string(),
                    );

                    diagnostics.push(Diagnostic::new(
                        self.id(),
                        self.name(),
                        "Lists should be followed by a blank line".to_string(),
                        last_item.line_number,
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
