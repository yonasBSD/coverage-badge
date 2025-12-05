# GitHub Action for Coverage Badge Generation

## Overview

Create a GitHub Action that makes it trivially easy to generate coverage badges in any repository. Users add a 4-line snippet to their workflow and get a shields.io-style SVG badge.

## User Experience

After running their coverage tool, users add:

```yaml
- name: Generate coverage badge
  uses: ozankasikci/rust-test-coverage-badge@v1
  with:
    coverage: ${{ steps.coverage.outputs.percentage }}
    output: assets/coverage.svg
```

The action:
1. Detects runner OS and architecture
2. Downloads the matching pre-built binary from GitHub Releases
3. Runs `coverage-badge -c <percentage> -o <output>`

The action does NOT commit anything - users handle that separately if desired.

## Action Inputs

| Input | Required | Description |
|-------|----------|-------------|
| `coverage` | Yes | Coverage percentage (0-100, decimals allowed) |
| `output` | Yes | Output path for the SVG file |

## Implementation

### File: `action.yml` (repo root)

Composite action with shell steps:

1. **Determine platform:** Map `runner.os` and `runner.arch` to release artifact names
   - `Linux` + `X64` → `coverage-badge-linux-x86_64`
   - `Linux` + `ARM64` → `coverage-badge-linux-aarch64`
   - `macOS` + `X64` → `coverage-badge-darwin-x86_64`
   - `macOS` + `ARM64` → `coverage-badge-darwin-arm64`
   - `Windows` + `X64` → `coverage-badge-windows-x86_64.exe`

2. **Download binary:** Fetch from `https://github.com/ozankasikci/rust-test-coverage-badge/releases/download/v{version}/...`

3. **Make executable:** `chmod +x` (non-Windows)

4. **Run:** Execute with provided inputs

### Version Resolution

The action needs to know which release to download. Options:

- **Hardcode version in action.yml:** Simple, but requires updating action.yml with each release
- **Use action ref as version:** If user specifies `@v1.2.0`, download that version's binary

Recommendation: Hardcode the version in `action.yml`. When releasing, update the version number and the `v1` tag together.

## Versioning Strategy

- **Release tags:** Semantic versions (`v0.2.0`, `v1.0.0`, etc.)
- **Action tags:** Floating major version tags (`v1`, `v2`)
- **Process:** When releasing `v1.2.0`:
  1. Create release with binaries
  2. Update version in `action.yml`
  3. Move `v1` tag to point to new release

## README Structure

1. **Hero:** Badge example + one-liner description
2. **Quick Start:** GitHub Action snippet (primary path)
3. **Full Example:** Complete workflow with coverage extraction + badge + commit
4. **CLI Usage:** Alternative for local use / custom setups
5. **Inputs Reference:** Table of action inputs
6. **Color Thresholds:** Existing table

## Full Workflow Example (for README)

```yaml
name: Coverage

on:
  push:
    branches: [main]

permissions:
  contents: write

jobs:
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # Your coverage tool here - this is just an example
      - name: Run tests with coverage
        id: coverage
        run: |
          # Replace with your actual coverage command
          # This example just sets a dummy value
          echo "percentage=85.5" >> $GITHUB_OUTPUT

      - name: Generate coverage badge
        uses: ozankasikci/rust-test-coverage-badge@v1
        with:
          coverage: ${{ steps.coverage.outputs.percentage }}
          output: assets/coverage.svg

      # Optional: commit the badge
      - name: Commit badge
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add assets/coverage.svg
          git diff --staged --quiet || git commit -m "Update coverage badge"
          git push
```

## Supported Platforms

Based on existing release workflow:

| OS | Architecture | Binary Name |
|----|--------------|-------------|
| Linux | x86_64 | `coverage-badge-linux-x86_64` |
| Linux | aarch64 | `coverage-badge-linux-aarch64` |
| macOS | x86_64 | `coverage-badge-darwin-x86_64` |
| macOS | arm64 | `coverage-badge-darwin-arm64` |
| Windows | x86_64 | `coverage-badge-windows-x86_64.exe` |

## Out of Scope

- External badge services (gist, S3, shields.io endpoints)
- Automatic committing (users handle this)
- Coverage extraction (language-specific, users handle this)
- Cargo install option in action (too slow for CI)
