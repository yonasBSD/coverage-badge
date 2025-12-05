# coverage-badge Design

A Rust CLI tool that generates shields.io-style SVG badges for test coverage.

## CLI Interface

```
coverage-badge --coverage <PERCENTAGE> --output <PATH>

Arguments:
  --coverage, -c <PERCENTAGE>  Coverage value (0-100, decimals allowed)
  --output, -o <PATH>          Output path for the SVG file

Examples:
  coverage-badge -c 85.5 -o assets/coverage.svg
  coverage-badge --coverage 42 --output .github/badges/coverage.svg
```

## Behavior

- Validates coverage is between 0-100 (inclusive)
- On invalid input: exit code 1 with helpful error message
- On success: writes SVG file, prints `Badge written to <path>`

## Badge Generation

### SVG Style

Shields.io flat style:
- Left side: label "coverage" with dark gray background (#555)
- Right side: percentage value with color based on thresholds
- Rounded corners, consistent height (~20px)
- Font: Verdana/DejaVu Sans

### Color Thresholds

| Coverage | Color  | Hex     |
|----------|--------|---------|
| < 50%    | Red    | #e05d44 |
| 50-79%   | Yellow | #dfb317 |
| ≥ 80%    | Green  | #4c1    |

### Percentage Display

- Show one decimal place if present, otherwise whole number
- Examples: `85%`, `72.5%`, `100%`

## Project Structure

```
coverage-badge/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry point, arg parsing
│   ├── lib.rs           # Public API (for testing & potential library use)
│   ├── badge.rs         # SVG generation logic
│   └── color.rs         # Color threshold logic
├── tests/
│   └── integration.rs   # End-to-end CLI tests
└── README.md
```

## Dependencies

- `clap` (with derive feature) — CLI argument parsing
- No other runtime dependencies

SVG generation is hand-rolled (simple template, no heavy XML libraries needed).

## Error Handling

### Invalid Input

| Input                  | Error Message                                              |
|------------------------|------------------------------------------------------------|
| Missing `--coverage`   | `error: required argument '--coverage <PERCENTAGE>' not provided` |
| Missing `--output`     | `error: required argument '--output <PATH>' not provided`  |
| Coverage < 0           | `error: coverage must be between 0 and 100, got: -5`       |
| Coverage > 100         | `error: coverage must be between 0 and 100, got: 150`      |
| Non-numeric coverage   | `error: invalid coverage value '<input>', expected a number` |

### File System Errors

| Scenario                     | Behavior                                                  |
|------------------------------|-----------------------------------------------------------|
| Parent directory doesn't exist | `error: cannot write to '<path>': directory does not exist` |
| No write permission          | `error: cannot write to '<path>': permission denied`      |
| File already exists          | Overwrite silently (badges are regenerated)               |

### Exit Codes

- `0` — Success
- `1` — Any error

## Future Expansion (Not in Initial Scope)

- Parse coverage from lcov/cobertura files via `--input <file>`
- Custom color thresholds via CLI flags
- Stdout output with `--output -`
