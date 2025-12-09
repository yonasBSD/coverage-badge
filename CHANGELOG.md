## [unreleased]

### 🚀 Features

- Auto-create output directory if it doesn't exist

### 🐛 Bug Fixes

- Pull --rebase before push to avoid conflicts
- Stash untracked files before rebase to avoid conflicts

### 🧪 Testing

- Refactor main.rs for better testability and add unit tests

### ⚙️ Miscellaneous Tasks

- Migrate to yonasBSD/coverage-badge
- Use local coverage badge.
- Add workflows.
- Bump version to 0.1.1
- Update Cargo.lock
- Bump version to 0.2.0
- Update Cargo.lock
- Update workflows.
- Upgrade codeql workflow.
- Clippy fixes.
- Update create-pull-request to v7.
- Update changelog (#1)
- Use yonasBSD/coverage-badge@v0
- Use yonasBSD/coverage-badge@v0
## [0.1.0] - 2025-12-05

### 🚀 Features

- Add GitHub Action for coverage badge generation
- Add optional commit support to action
- Add coverage_level function (to test commit option)

### 🚜 Refactor

- Use own GitHub Action for coverage badge

### 📚 Documentation

- Update README with GitHub Action usage
- Tidy README and add table of contents
- Add CI, crates.io, and license badges

### 🧪 Testing

- Add more unit tests to increase coverage
- Add tests for coverage_level function
