# CI/CD pipeline

## Pipeline stages (GitHub Actions)

```
lint (clippy + fmt) ──▶ test ──▶ build ──▶ coverage ──▶ security (cargo-audit) ──▶ release
```

Triggered on:
- Push to `develop`, `main`, `feature/*`, `release/*`, `hotfix/*`
- All pull requests targeting `develop` or `main`
- Tags matching `v*`

## Caching

Dependencies are cached via `Swatinem/rust-cache@v2` across all jobs.
Keyed by `Cargo.lock` hash — cache is shared across branches with the
same lockfile, cutting CI time by ~60%.

## Quality gates

| Check      | Required for merge | Tool               |
| ---------- | ------------------ | ------------------ |
| Formatting | yes                | `cargo fmt --check` |
| Lint       | yes                | `cargo clippy`     |
| Tests      | yes                | `cargo test`       |
| Build      | yes                | `cargo build`      |
| Coverage   | advisory only       | `cargo tarpaulin`  |
| Security   | advisory only       | `cargo audit`      |

## Local setup

Install git hooks:

```bash
git config core.hooksPath .githooks
```

## Coverage

Coverage is collected with `tarpaulin` in the `coverage` job and uploaded
as a CI artifact (`cobertura.xml`). Target: ≥70%.
