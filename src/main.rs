use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;

/// Generate a shields.io-style coverage badge as an SVG file.
#[derive(Parser)]
#[command(name = "coverage-badge")]
#[command(about = "Generate a coverage badge SVG", long_about = None)]
struct Cli {
    /// Coverage percentage (0-100)
    #[arg(short, long)]
    coverage: f64,

    /// Output path for the SVG file
    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // Validate coverage range
    if cli.coverage < 0.0 || cli.coverage > 100.0 {
        eprintln!("error: coverage must be between 0 and 100, got: {}", cli.coverage);
        process::exit(1);
    }

    // Generate the badge
    let svg = coverage_badge::generate_badge(cli.coverage);

    // Write to file
    if let Some(parent) = cli.output.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            eprintln!("error: cannot write to '{}': directory does not exist", cli.output.display());
            process::exit(1);
        }
    }

    match fs::write(&cli.output, &svg) {
        Ok(_) => println!("Badge written to {}", cli.output.display()),
        Err(e) => {
            eprintln!("error: cannot write to '{}': {}", cli.output.display(), e);
            process::exit(1);
        }
    }
}
