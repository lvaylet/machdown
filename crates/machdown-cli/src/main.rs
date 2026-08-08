use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use machdown_fixer::fix_file_in_place;
use machdown_parser::parse;
use machdown_reporter::print_diagnostics_pretty;
use machdown_rules::{default_rules, Diagnostic};

#[derive(Parser)]
#[command(
    name = "machdown",
    version,
    about = "High-performance Markdown linter and auto-fixer"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check markdown files for violations
    Check {
        /// File or directory paths to check
        #[arg(required = true)]
        paths: Vec<PathBuf>,
    },
    /// Fix auto-fixable violations in markdown files
    Fix {
        /// File or directory paths to fix
        #[arg(required = true)]
        paths: Vec<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let rules = default_rules();

    match cli.command {
        Commands::Check { paths } => {
            let mut all_diagnostics: Vec<Diagnostic> = Vec::new();

            for path in paths {
                if !path.exists() {
                    eprintln!("Error: file or directory not found: {}", path.display());
                    return ExitCode::from(2);
                }

                let content = match std::fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(err) => {
                        eprintln!("Error reading file {}: {}", path.display(), err);
                        return ExitCode::from(2);
                    }
                };

                let doc = parse(&content);
                for rule in &rules {
                    let diags = rule.check(&doc);
                    for d in diags {
                        all_diagnostics.push(d.with_file_path(path.clone()));
                    }
                }
            }

            if !all_diagnostics.is_empty() {
                print_diagnostics_pretty(&all_diagnostics);
                ExitCode::from(1)
            } else {
                ExitCode::from(0)
            }
        }
        Commands::Fix { paths } => {
            let mut unfixable_diagnostics: Vec<Diagnostic> = Vec::new();

            for path in paths {
                if !path.exists() {
                    eprintln!("Error: file or directory not found: {}", path.display());
                    return ExitCode::from(2);
                }

                if let Err(err) = fix_file_in_place(&path, &rules) {
                    eprintln!("Error fixing file {}: {}", path.display(), err);
                    return ExitCode::from(2);
                }

                let content = match std::fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(err) => {
                        eprintln!("Error reading file {}: {}", path.display(), err);
                        return ExitCode::from(2);
                    }
                };

                let doc = parse(&content);
                for rule in &rules {
                    let diags = rule.check(&doc);
                    for d in diags {
                        unfixable_diagnostics.push(d.with_file_path(path.clone()));
                    }
                }
            }

            if !unfixable_diagnostics.is_empty() {
                print_diagnostics_pretty(&unfixable_diagnostics);
                ExitCode::from(1)
            } else {
                ExitCode::from(0)
            }
        }
    }
}
