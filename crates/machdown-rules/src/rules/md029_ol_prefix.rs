use machdown_parser::Document;

use crate::{Diagnostic, Edit, Rule, Severity, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OlPrefixStyle {
    One,
    Ordered,
    #[default]
    OneOrOrdered,
    Zero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MD029OlPrefix {
    pub style: OlPrefixStyle,
}

impl MD029OlPrefix {
    pub fn new(style: OlPrefixStyle) -> Self {
        Self { style }
    }
}

impl Rule for MD029OlPrefix {
    fn id(&self) -> &'static str {
        "MD029"
    }

    fn name(&self) -> &'static str {
        "ol-prefix"
    }

    fn description(&self) -> &'static str {
        "Ordered list item prefix numbering consistency"
    }

    fn check(&self, doc: &Document) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        for list in doc.lists() {
            let ol_items: Vec<_> = list.items.iter().filter(|it| it.is_ordered).collect();
            if ol_items.is_empty() {
                continue;
            }

            let effective_style = match self.style {
                OlPrefixStyle::One => OlPrefixStyle::One,
                OlPrefixStyle::Ordered => OlPrefixStyle::Ordered,
                OlPrefixStyle::Zero => OlPrefixStyle::Zero,
                OlPrefixStyle::OneOrOrdered => {
                    if ol_items.len() > 1 && ol_items[1].number == Some(1) {
                        OlPrefixStyle::One
                    } else {
                        OlPrefixStyle::Ordered
                    }
                }
            };

            let expected_seq = ol_items[0].number.unwrap_or(1);
            let delim = ol_items[0].delimiter.unwrap_or('.');

            for (idx, item) in ol_items.iter().enumerate() {
                let expected_num = match effective_style {
                    OlPrefixStyle::One => 1,
                    OlPrefixStyle::Zero => 0,
                    OlPrefixStyle::Ordered => {
                        if idx == 0 {
                            expected_seq
                        } else {
                            expected_seq + idx as u64
                        }
                    }
                    OlPrefixStyle::OneOrOrdered => unreachable!(),
                };

                let item_num = item.number.unwrap_or(1);
                let item_delim = item.delimiter.unwrap_or(delim);

                if item_num != expected_num || item_delim != delim {
                    let expected_marker = format!("{}{}", expected_num, delim);
                    let edit = Edit::new(
                        Span::new(
                            item.line_number,
                            item.marker_span.start_col,
                            item.marker_span.start_byte,
                            item.marker_span.end_byte,
                        ),
                        expected_marker.clone(),
                    );

                    diagnostics.push(Diagnostic::new(
                        self.id(),
                        self.name(),
                        format!(
                            "Ordered list item prefix expected '{}', found '{}'",
                            expected_marker, item.marker
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
