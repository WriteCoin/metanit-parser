# AGENTS.md

## Project

Rust library for parsing [metanit.com](https://metanit.com) tutorial pages (`metanit-parser`).

## Build & Test

```bash
cargo check        # no warnings expected
cargo test         # 12 unit + 5 integration tests
cargo clippy       # clean (allow one dead_code warning in client.rs CacheEntry.expires_at)
cargo fmt --check
cargo build --release
```

## Git Workflow

Git Flow: `main` (stable) ← `develop` (integration) ← `feature/*`, `release/*`, `hotfix/*`.

- **Protected branches**: `main`, `develop` — no direct push, only PR/MR with CI + review.
- **Feature branches**: branch from `develop`, merge back via `--no-ff`.
- **Release**: `release/v*` branch from `develop`, merge to `main` + tag, merge back to `develop`.
- **Commit style**: Conventional Commits (`feat:`, `fix:`, `docs:`, `test:`, `ci:`, `chore:`, etc.).

## Git hooks (`.githooks/`)

```
git config core.hooksPath .githooks
```

- `pre-commit`: `cargo fmt --check` + `cargo clippy`
- `commit-msg`: validates Conventional Commits format
- `pre-push`: `cargo test`

## CI/CD (`.github/workflows/ci.yml`)

Pipeline: `lint → test → build → security → release` (on tag `v*`).
Triggered on push/PR to `develop`, `main`, `feature/*`, `release/*`, `hotfix/*`, and tags `v*`.

## Architecture

```
src/
├── lib.rs       — entry point, public re-exports
├── client.rs    — HTTP transport (reqwest, blocking), retry + cache
├── parser.rs    — HTML parsing (scraper crate), code blocks + menu extraction
├── models.rs    — Page, CodeBlock, MenuItem, PageSummary
└── error.rs     — Error/ErrorKind with From impls for reqwest + url
```

Key design: blocking HTTP, no async in public API, immutable owned types via serde.

## Tests

- Unit tests inline in `src/parser.rs` and `src/lib.rs`
- Integration tests in `tests/integration_tests.rs` (mock HTML, serialization roundtrips)

Known quirks:
- `expires_at` field in `CacheEntry` is stored but not checked — cache is write-only for expiration
- HTTP client uses `rustls-tls` (not native-tls, which requires OpenSSL)
