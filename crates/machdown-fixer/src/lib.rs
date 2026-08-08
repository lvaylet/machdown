use std::fs;
use std::io::Write;
use std::path::Path;

use machdown_parser::parse;
use machdown_rules::{Edit, Rule};

/// Apply a slice of non-overlapping edits to source text.
/// Edits are applied in reverse order of start byte to ensure byte offsets remain valid.
pub fn apply_edits(source: &str, edits: &[Edit]) -> String {
    if edits.is_empty() {
        return source.to_string();
    }

    let mut sorted_edits = edits.to_vec();
    sorted_edits.sort_by_key(|e| std::cmp::Reverse(e.span().start_byte()));

    let mut result = source.to_string();
    let mut last_start = usize::MAX;

    for edit in sorted_edits {
        let span = edit.span();
        let start = span.start_byte();
        let end = span.end_byte();

        // Ensure non-overlapping edits
        if end > last_start {
            continue;
        }
        if start > result.len() || end > result.len() || start > end {
            continue;
        }

        result.replace_range(start..end, edit.replacement());
        last_start = start;
    }

    result
}

/// Run iterative fix passes on source text using the given rules.
/// Returns the fixed string and the total number of fixes applied.
pub fn fix_str(source: &str, rules: &[Box<dyn Rule>]) -> (String, usize) {
    let mut current = source.to_string();
    let mut total_fixes = 0;
    let max_passes = 10;

    for _ in 0..max_passes {
        let doc = parse(&current);
        let mut edits = Vec::new();

        for rule in rules {
            let diagnostics = rule.check(&doc);
            for d in diagnostics {
                if let Some(fix) = d.fix() {
                    edits.push(fix.clone());
                }
            }
        }

        if edits.is_empty() {
            break;
        }

        let edits_count = edits.len();
        let next = apply_edits(&current, &edits);
        if next == current {
            break;
        }

        total_fixes += edits_count;
        current = next;
    }

    (current, total_fixes)
}

/// Atomically fix a file in place using a temporary file in the same directory.
/// Returns the number of fixes applied.
pub fn fix_file_in_place(path: &Path, rules: &[Box<dyn Rule>]) -> std::io::Result<usize> {
    let content = fs::read_to_string(path)?;
    let (fixed, count) = fix_str(&content, rules);

    if count > 0 && fixed != content {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let mut temp_file = tempfile::Builder::new()
            .prefix(".machdown-fix-")
            .tempfile_in(parent)?;

        temp_file.write_all(fixed.as_bytes())?;
        temp_file.flush()?;
        temp_file.persist(path).map_err(|e| e.error)?;
    }

    Ok(count)
}
