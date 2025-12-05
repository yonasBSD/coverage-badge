# GitHub Action Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Create a GitHub Action that downloads the coverage-badge binary and generates an SVG badge from a coverage percentage.

**Architecture:** Composite action with shell steps that detects OS/arch, downloads the pre-built binary tarball from GitHub Releases, extracts it, and runs the CLI with user-provided inputs.

**Tech Stack:** GitHub Actions (composite), bash, curl

---

## Task 1: Create the Composite Action

**Files:**
- Create: `action.yml`

**Step 1: Create action.yml with inputs and platform detection**

```yaml
name: 'Coverage Badge'
description: 'Generate a shields.io-style SVG coverage badge'
author: 'Ozan Kaşıkçı'

branding:
  icon: 'award'
  color: 'green'

inputs:
  coverage:
    description: 'Coverage percentage (0-100, decimals allowed)'
    required: true
  output:
    description: 'Output path for the SVG file'
    required: true

runs:
  using: 'composite'
  steps:
    - name: Determine platform
      id: platform
      shell: bash
      run: |
        case "${{ runner.os }}-${{ runner.arch }}" in
          Linux-X64)
            echo "target=x86_64-unknown-linux-gnu" >> $GITHUB_OUTPUT
            ;;
          macOS-X64)
            echo "target=x86_64-apple-darwin" >> $GITHUB_OUTPUT
            ;;
          macOS-ARM64)
            echo "target=aarch64-apple-darwin" >> $GITHUB_OUTPUT
            ;;
          *)
            echo "::error::Unsupported platform: ${{ runner.os }}-${{ runner.arch }}"
            exit 1
            ;;
        esac

    - name: Download and extract binary
      shell: bash
      run: |
        VERSION="0.1.0"
        TARGET="${{ steps.platform.outputs.target }}"
        URL="https://github.com/yonasBSD/coverage-badge/releases/download/v${VERSION}/coverage-badge-${TARGET}.tar.gz"

        echo "Downloading from: $URL"
        curl -fsSL "$URL" -o coverage-badge.tar.gz
        tar -xzf coverage-badge.tar.gz
        chmod +x coverage-badge

    - name: Generate badge
      shell: bash
      run: |
        ./coverage-badge -c "${{ inputs.coverage }}" -o "${{ inputs.output }}"
        echo "Badge generated at ${{ inputs.output }}"

    - name: Cleanup
      shell: bash
      run: |
        rm -f coverage-badge coverage-badge.tar.gz
```

**Step 2: Verify the file is valid YAML**

Run: `python3 -c "import yaml; yaml.safe_load(open('action.yml'))"`
Expected: No output (valid YAML)

**Step 3: Commit**

```bash
git add action.yml
git commit -m "feat: add GitHub Action for coverage badge generation"
```

---

## Task 2: Update README with Action Usage

**Files:**
- Modify: `README.md`

**Step 1: Rewrite README with action-first approach**

```markdown
# coverage-badge

![Coverage](assets/coverage.svg)

A GitHub Action and CLI tool that generates shields.io-style SVG badges for test coverage.

## Quick Start (GitHub Action)

Add this step to your workflow after running your coverage tool:

```yaml
- name: Generate coverage badge
  uses: yonasBSD/coverage-badge@v1
  with:
    coverage: ${{ steps.coverage.outputs.percentage }}
    output: assets/coverage.svg
```

Then reference the badge in your README:

```markdown
![Coverage](assets/coverage.svg)
```

## Full Workflow Example

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

      # Replace with your actual coverage tool
      - name: Run tests with coverage
        id: coverage
        run: |
          # Example: extract coverage from your tool's output
          # For tarpaulin: jq '.coverage' tarpaulin-report.json
          # For pytest-cov: grep -oP 'TOTAL.*\s+\K\d+' coverage.txt
          echo "percentage=85.5" >> $GITHUB_OUTPUT

      - name: Generate coverage badge
        uses: yonasBSD/coverage-badge@v1
        with:
          coverage: ${{ steps.coverage.outputs.percentage }}
          output: assets/coverage.svg

      # Optional: commit the updated badge
      - name: Commit badge
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add assets/coverage.svg
          git diff --staged --quiet || git commit -m "Update coverage badge"
          git push
```

## Action Inputs

| Input | Required | Description |
|-------|----------|-------------|
| `coverage` | Yes | Coverage percentage (0-100, decimals allowed) |
| `output` | Yes | Output path for the SVG file |

## CLI Usage

For local development or custom setups, you can use the CLI directly.

### Install via Cargo

```bash
cargo install coverage-badge
```

### Install from Releases

Download the binary for your platform from [Releases](https://github.com/yonasBSD/coverage-badge/releases).

### Run

```bash
coverage-badge --coverage 85 --output assets/coverage.svg
```

## Color Thresholds

| Coverage | Color |
|----------|-------|
| < 50% | Red |
| 50-79% | Yellow |
| ≥ 80% | Green |

## License

MIT
```

**Step 2: Commit**

```bash
git add README.md
git commit -m "docs: update README with GitHub Action usage"
```

---

## Task 3: Test the Action Locally

**Files:**
- None (manual verification)

**Step 1: Verify action.yml syntax**

Run: `cat action.yml | head -20`
Expected: Valid YAML structure visible

**Step 2: Test that current binary works**

Run: `cargo build --release && ./target/release/coverage-badge -c 75.5 -o /tmp/test-badge.svg && cat /tmp/test-badge.svg | head -5`
Expected: SVG content starting with `<svg`

**Step 3: Verify release assets exist**

Run: `curl -sI "https://github.com/yonasBSD/coverage-badge/releases/download/v0.1.0/coverage-badge-x86_64-unknown-linux-gnu.tar.gz" | head -1`
Expected: `HTTP/2 302` or `HTTP/2 200` (redirect to asset)

---

## Task 4: Create and Push v1 Tag

**Files:**
- None (git operations only)

**Step 1: Verify all changes are committed**

Run: `git status`
Expected: Clean working tree

**Step 2: Create v1 tag pointing to HEAD**

Run: `git tag v1`
Expected: No output (tag created)

**Step 3: Push commits and tag**

Run: `git push && git push origin v1`
Expected: Push succeeds

---

## Task 5: Update This Repo's Workflow to Use the Action

**Files:**
- Modify: `.github/workflows/coverage.yml`

**Step 1: Update workflow to use own action**

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
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Run tests with coverage
        run: cargo tarpaulin --out json --output-dir target/coverage

      - name: Extract coverage percentage
        id: coverage
        run: |
          COVERAGE=$(cat target/coverage/tarpaulin-report.json | jq '.coverage')
          echo "percentage=$COVERAGE" >> $GITHUB_OUTPUT
          echo "Coverage: $COVERAGE%"

      - name: Generate badge
        uses: yonasBSD/coverage-badge@v1
        with:
          coverage: ${{ steps.coverage.outputs.percentage }}
          output: assets/coverage.svg

      - name: Commit badge
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add assets/coverage.svg
          git diff --staged --quiet || git commit -m "Update coverage badge"
          git push
```

**Step 2: Commit**

```bash
git add .github/workflows/coverage.yml
git commit -m "refactor: use own GitHub Action for coverage badge"
```

**Step 3: Push to trigger workflow**

Run: `git push`
Expected: Push succeeds, workflow will run and test the action

---

## Summary

After completing all tasks:

1. ✅ `action.yml` exists at repo root
2. ✅ README documents action usage first, CLI second
3. ✅ `v1` tag exists for action consumers
4. ✅ This repo's own workflow uses the action (dogfooding)
5. ✅ Users can now integrate with a 4-line snippet
