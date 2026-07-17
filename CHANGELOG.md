# Changelog

## [1.1.0] — 2026-07-18

### Added
- Code block language detection and extraction functions
- Menu parser for navigation structure
- Content extraction with full body text parsing
- Comprehensive test suites (12 unit + 6 integration tests)
- thiserror integration for better error ergonomics

### Fixed
- Robust error handling for missing titles and malformed HTML

## [1.0.0] — 2026-07-18

### Added
- MetanitClient — HTTP client with caching and retry
- Page, CodeBlock, MenuItem — structured data models
- HTML parser with title, content, and code block extraction
- Menu and navigation parsing
- Code block language detection and counting
- Comprehensive unit and integration tests
- CI/CD pipeline with lint, test, build, and security stages
- Git hooks for pre-commit, commit-msg, and pre-push
