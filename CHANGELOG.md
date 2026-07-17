# Changelog

## [1.1.0] — 2026-07-18

### Added
- Pagination support for multi-page sections
- Code block language detection
- Caching layer in HTTP client

### Fixed
- UTF-8 encoding handling for Cyrillic content
- Panic on malformed HTML menu items

## [1.0.0] — 2026-07-18

### Added
- `MetanitClient` — HTTP client with retry logic
- `Page` struct — parsed page with title, content, and code blocks
- `Menu` parser — extracts navigation tree from sidebar
- `Error` types — typed errors for HTTP, parse, and encoding failures
- Unit and integration test suites
- CI/CD pipeline with lint, test, build, and security stages
