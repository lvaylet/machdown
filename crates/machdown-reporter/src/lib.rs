use colored::*;
use machdown_rules::{Diagnostic, Severity};

/// Format a diagnostic as a plain uncolored string.
pub fn format_diagnostic_plain(diagnostic: &Diagnostic) -> String {
    let file = diagnostic
        .file_path()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "<stdin>".to_string());

    let severity_str = match diagnostic.severity() {
        Severity::Warning => "warning",
        Severity::Error => "error",
    };

    format!(
        "{}:{}:{}: {}: [{}] {}",
        file,
        diagnostic.line(),
        diagnostic.column(),
        severity_str,
        diagnostic.rule_id(),
        diagnostic.message()
    )
}

/// Format a diagnostic with ANSI color coding.
pub fn format_diagnostic_pretty(diagnostic: &Diagnostic) -> String {
    let file = diagnostic
        .file_path()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "<stdin>".to_string());

    let file_loc = format!("{}:{}:{}", file, diagnostic.line(), diagnostic.column()).bold();

    let severity_styled = match diagnostic.severity() {
        Severity::Warning => "warning".yellow().bold(),
        Severity::Error => "error".red().bold(),
    };

    let rule_id = format!("[{}]", diagnostic.rule_id()).cyan().bold();

    format!(
        "{}: {}: {} {}",
        file_loc,
        severity_styled,
        rule_id,
        diagnostic.message()
    )
}

/// Print diagnostics to stdout using pretty terminal formatting.
pub fn print_diagnostics_pretty(diagnostics: &[Diagnostic]) {
    for d in diagnostics {
        println!("{}", format_diagnostic_pretty(d));
    }
}
