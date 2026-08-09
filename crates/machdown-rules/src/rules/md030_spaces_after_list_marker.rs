use machdown_parser::Document;

use crate::{Diagnostic, Edit, Rule, Severity, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MD030SpacesAfterListMarker {
    pub ul_single: usize,
    pub ol_single: usize,
    pub ul_multi: usize,
    pub ol_multi: usize,
}

impl Default for MD030SpacesAfterListMarker {
    fn default() -> Self {
        Self {
            ul_single: 1,
            ol_single: 1,
            ul_multi: 1,
            ol_multi: 1,
        }
    }
}

impl Rule for MD030SpacesAfterListMarker {
    fn id(&self) -> &'static str {
        "MD030"
    }

    fn name(&self) -> &'static str {
        "spaces-after-list-marker"
    }

    fn description(&self) -> &'static str {
        "Spaces after list markers"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for item in doc.list_items() {
            let expected_spaces = if item.is_ordered {
                self.ol_single
            } else {
                self.ul_single
            };

            if item.spaces_after_marker != expected_spaces {
                let start_byte = item.marker_span.end_byte;
                let end_byte = item.marker_span.end_byte + item.spaces_after_marker;
                let col = item.marker_span.end_col;

                let edit = Edit::new(
                    Span::new(item.line_number, col, start_byte, end_byte),
                    " ".repeat(expected_spaces),
                );

                diagnostics.push(Diagnostic::new(
                    self.id(),
                    self.name(),
                    format!(
                        "Spaces after list marker expected {}, found {}",
                        expected_spaces, item.spaces_after_marker
                    ),
                    item.line_number,
                    col,
                    Severity::Warning,
                    Some(edit),
                ));
            }
        }

        diagnostics
    }
}
