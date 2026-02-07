#![allow(unused_assignments)]

use clap::Parser;
use core_lib::{
    ast::Form,
    vis_parser::{self, ParserError},
};
use indexmap::IndexMap;
use miette::{GraphicalReportHandler, NamedSource, Report, SourceSpan};
use std::path::Path;
use tracing::Level;

use crate::templater::templater;

mod config;
mod templater;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = false)]
    check: bool,

    #[arg(long, default_value_t = String::from("./vis.config.json"))]
    config: String,
}

fn main() {
    let args = Args::parse();

    let config_contents =
        std::fs::read_to_string(&args.config).expect("Failed to read config file");
    let config: config::Config =
        serde_json::from_str(&config_contents).expect("Failed to parse config file");

    if !std::path::Path::new(&args.config).exists() {
        eprintln!("Config file not found: {}", args.config);
        std::process::exit(2);
    }

    // Always run the check
    let result = run_check(&config.input);
    if result.is_err() || args.check {
        return;
    }

    templater(result.unwrap(), config);

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
}

fn run_check(input: &String) -> Result<IndexMap<String, IndexMap<String, Form>>, ()> {
    // check if file exists

    let handler = GraphicalReportHandler::new();
    let mut had_errors = false;
    let mut results: IndexMap<String, IndexMap<String, Form>> = IndexMap::new();
    for entry in std::fs::read_dir(&input).expect("Failed to read input directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("vis") {
            println!("Checking file \"{}\"", path.display());
            let contents = std::fs::read_to_string(&path).expect("Failed to read .vis file");
            let parse_result = vis_parser::parse_vis(&contents);
            match parse_result {
                Ok(file) => {
                    println!("  OK");
                    let file_name_without_ext = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default()
                        .to_string();
                    results.insert(file_name_without_ext, file);
                }
                Err(errors) => {
                    had_errors = true;
                    println!("  {} error(s)", errors.len());
                    render_parse_errors(&handler, &path, &contents, &errors);
                }
            }
        }
    }

    if had_errors {
        Err(())
    } else {
        Ok(results)
    }
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("{message}")]
#[diagnostic(code(mohazi::parse))]
#[allow(unused_assignments)]
struct VisDiagnostic {
    message: String,

    #[source_code]
    src: NamedSource<String>,

    #[label("here")]
    span: SourceSpan,
}

fn render_parse_errors(
    handler: &GraphicalReportHandler,
    path: &Path,
    source: &str,
    errors: &[ParserError],
) {
    for error in errors {
        let span = line_span(source, error.line as usize)
            .map(|(offset, len)| SourceSpan::new(offset.into(), len.into()))
            .unwrap_or_else(|| SourceSpan::new(0usize.into(), source.len().into()));

        let diag = VisDiagnostic {
            message: error.message.clone(),
            src: NamedSource::new(path.display().to_string(), source.to_string()),
            span,
        };

        let report = Report::new(diag);
        let mut rendered = String::new();
        if handler
            .render_report(&mut rendered, report.as_ref())
            .is_ok()
        {
            eprintln!("{rendered}");
        } else {
            // Fallback (should be rare)
            eprintln!("{}:{}: {}", path.display(), error.line + 1, error.message);
        }
    }
}

fn line_span(source: &str, line0: usize) -> Option<(usize, usize)> {
    let bytes = source.as_bytes();

    let mut current_line = 0usize;
    let mut start = 0usize;

    if line0 == 0 {
        start = 0;
    } else {
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'\n' {
                current_line += 1;
                if current_line == line0 {
                    start = i + 1;
                    break;
                }
            }
        }

        if current_line != line0 {
            return None;
        }
    }

    if start > bytes.len() {
        return None;
    }

    let mut end = bytes.len();
    for (i, &b) in bytes.iter().enumerate().skip(start) {
        if b == b'\n' {
            end = i;
            break;
        }
    }

    // Avoid highlighting a trailing '\r' on Windows line endings.
    if end > start && bytes[end - 1] == b'\r' {
        end -= 1;
    }

    Some((start, end.saturating_sub(start)))
}
