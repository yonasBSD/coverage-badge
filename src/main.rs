use clap::Parser;
use std::fs;
use std::path::Path;
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

/// Validates coverage is within valid range (0-100).
fn validate_coverage(coverage: f64) -> Result<(), String> {
    if coverage < 0.0 || coverage > 100.0 {
        Err(format!("coverage must be between 0 and 100, got: {}", coverage))
    } else {
        Ok(())
    }
}

/// Validates that the output directory exists.
fn validate_output_path(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(format!("cannot write to '{}': directory does not exist", path.display()));
        }
    }
    Ok(())
}

/// Writes the badge to the specified path.
fn write_badge(path: &Path, svg: &str) -> Result<(), String> {
    fs::write(path, svg).map_err(|e| format!("cannot write to '{}': {}", path.display(), e))
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = validate_coverage(cli.coverage) {
        eprintln!("error: {}", e);
        process::exit(1);
    }

    let svg = coverage_badge::generate_badge(cli.coverage);

    if let Err(e) = validate_output_path(&cli.output) {
        eprintln!("error: {}", e);
        process::exit(1);
    }

    match write_badge(&cli.output, &svg) {
        Ok(_) => println!("Badge written to {}", cli.output.display()),
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validate_coverage_valid() {
        assert!(validate_coverage(0.0).is_ok());
        assert!(validate_coverage(50.0).is_ok());
        assert!(validate_coverage(100.0).is_ok());
        assert!(validate_coverage(75.5).is_ok());
    }

    #[test]
    fn test_validate_coverage_invalid() {
        assert!(validate_coverage(-1.0).is_err());
        assert!(validate_coverage(101.0).is_err());
        assert!(validate_coverage(-0.1).is_err());
        assert!(validate_coverage(100.1).is_err());
    }

    #[test]
    fn test_validate_coverage_error_message() {
        let err = validate_coverage(150.0).unwrap_err();
        assert!(err.contains("150"));
        assert!(err.contains("between 0 and 100"));
    }

    #[test]
    fn test_validate_output_path_exists() {
        // Current directory should exist
        let path = PathBuf::from("test.svg");
        assert!(validate_output_path(&path).is_ok());
    }

    #[test]
    fn test_validate_output_path_missing_dir() {
        let path = PathBuf::from("nonexistent/dir/test.svg");
        assert!(validate_output_path(&path).is_err());
    }

    #[test]
    fn test_validate_output_path_error_message() {
        let path = PathBuf::from("missing/dir/badge.svg");
        let err = validate_output_path(&path).unwrap_err();
        assert!(err.contains("directory does not exist"));
    }

    #[test]
    fn test_write_badge_success() {
        let path = PathBuf::from("target/test-write-badge.svg");
        let svg = "<svg>test</svg>";
        assert!(write_badge(&path, svg).is_ok());

        // Verify content
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, svg);

        // Cleanup
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_write_badge_missing_dir() {
        let path = PathBuf::from("nonexistent/dir/badge.svg");
        let svg = "<svg>test</svg>";
        let result = write_badge(&path, svg);
        assert!(result.is_err());
    }
}
