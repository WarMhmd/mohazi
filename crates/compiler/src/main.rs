#![allow(unused_assignments)]

use clap::{CommandFactory, Parser, Subcommand};
use core_lib::{
    ast::Form,
    parser::{self, ParserError},
};
use directories::ProjectDirs;
use indexmap::IndexMap;
use miette::{GraphicalReportHandler, NamedSource, Report, SourceSpan};
use std::{fs::File, path::Path};
use tracing::{info, Level};

use crate::templater::templater;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, util::SubscriberInitExt, EnvFilter};

mod config;
mod templater;

#[derive(Parser)]
#[command(name = "MoHaZi", about = "MoHaZi Verification logic builder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long, default_value_t = false)]
        debug: bool,

        #[arg(long, default_value_t = false)]
        check: bool,

        #[arg(long, default_value_t = String::from("./mhz.config.json"))]
        config: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            debug,
            check,
            config,
        } => {
            let config_path = config
                .rsplit_once("mhz.config.json")
                .expect("Invalid config file name");

            let config_contents =
                std::fs::read_to_string(&config).expect("Failed to read config file");
            let configurations: config::Config =
                serde_json::from_str(&config_contents).expect("Failed to parse config file");

            if !std::path::Path::new(&config).exists() {
                eprintln!("Config file not found: {}", config);
                std::process::exit(2);
            }

            // Always run the check
            let input_path = format!("{}{}", config_path.0, configurations.input);
            println!("input path: {}", input_path);
            let result = run_check(&input_path);
            if result.is_err() || check {
                return;
            }

            let proj = ProjectDirs::from("com", "verify", "verify").unwrap();
            let log_dir: &Path = proj
                .state_dir()
                .or_else(|| Some(proj.cache_dir()))
                .or_else(|| Some(proj.data_dir()))
                .unwrap_or_else(|| Path::new("./logs"));

            std::fs::create_dir_all(log_dir).expect("failed to create log directory");

            // Add date and time to log file name
            let file = File::create(log_dir.join(format!(
                "{}-{}.log",
                chrono::Utc::now().date_naive(),
                chrono::Utc::now().time().format("%H-%M-%S")
            )))
            .expect("Failed to create log file");

            let stdout_layer = fmt::layer().with_writer(std::io::stdout).with_filter(
                tracing_subscriber::EnvFilter::new(if debug { "debug" } else { "off" }),
            );

            let file_layer = fmt::layer().with_writer(file).with_ansi(false);

            tracing_subscriber::registry()
                .with(file_layer)
                .with(stdout_layer)
                .init();

            // info!("Starting Mohazi compiler with config: {:?}", config);

            templater(result.unwrap(), configurations);
        }
    }
}

fn run_check(input: &String) -> Result<IndexMap<String, IndexMap<String, Form>>, ()> {
    // check if file exists

    let handler = GraphicalReportHandler::new();
    let mut had_errors = false;
    let mut results: IndexMap<String, IndexMap<String, Form>> = IndexMap::new();
    for entry in std::fs::read_dir(&input).expect("Failed to read input directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("mhz") {
            println!("Checking file \"{}\"", path.display());
            let contents = std::fs::read_to_string(&path).expect("Failed to read .mhz file");
            let parse_result = parser::parse_mhz(&contents);
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
struct MhzDiagnostic {
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

        let diag = MhzDiagnostic {
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
