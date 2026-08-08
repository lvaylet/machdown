use std::path::PathBuf;

use machdown_parser::Document;

pub mod rules;
pub use rules::*;

/// Severity of a diagnostic violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Warning,
    Error,
}

/// A span of source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    line: usize,
    column: usize,
    start_byte: usize,
    end_byte: usize,
}

impl Span {
    pub fn new(line: usize, column: usize, start_byte: usize, end_byte: usize) -> Self {
        Self {
            line,
            column,
            start_byte,
            end_byte,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn start_byte(&self) -> usize {
        self.start_byte
    }

    pub fn end_byte(&self) -> usize {
        self.end_byte
    }
}

/// A text replacement edit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    span: Span,
    replacement: String,
}

impl Edit {
    pub fn new(span: Span, replacement: String) -> Self {
        Self { span, replacement }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

/// Diagnostic representing a lint violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    file_path: Option<PathBuf>,
    rule_id: &'static str,
    rule_name: &'static str,
    message: String,
    line: usize,
    column: usize,
    severity: Severity,
    fix: Option<Edit>,
}

impl Diagnostic {
    pub fn new(
        rule_id: &'static str,
        rule_name: &'static str,
        message: String,
        line: usize,
        column: usize,
        severity: Severity,
        fix: Option<Edit>,
    ) -> Self {
        Self {
            file_path: None,
            rule_id,
            rule_name,
            message,
            line,
            column,
            severity,
            fix,
        }
    }

    pub fn with_file_path(mut self, path: PathBuf) -> Self {
        self.file_path = Some(path);
        self
    }

    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn rule_id(&self) -> &'static str {
        self.rule_id
    }

    pub fn rule_name(&self) -> &'static str {
        self.rule_name
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn severity(&self) -> Severity {
        self.severity
    }

    pub fn fix(&self) -> Option<&Edit> {
        self.fix.as_ref()
    }

    pub fn is_fixable(&self) -> bool {
        self.fix.is_some()
    }
}

/// Common trait implemented by all lint rules.
pub trait Rule: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn check(&self, doc: &Document) -> Vec<Diagnostic>;
}

/// Returns a default vector of all enabled rules.
pub fn default_rules() -> Vec<Box<dyn Rule>> {
    vec![Box::new(MD009TrailingWhitespace)]
}
