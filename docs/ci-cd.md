# CI/CD pipeline

## Pipeline stages (GitHub Actions)

```
lint (clippy + fmt) ──▶ test ──▶ build ──▶ security (cargo-audit) ──▶ release
```

Triggered on:
- Push to `develop`, `main`, `feature/*`, `release/*`, `hotfix/*`
- All pull requests targeting `develop` or `main`
- Tags matching `v*`

## Quality gates

| Check      | Required for merge | Tool               |
| ---------- | ------------------ | ------------------ |
| Formatting | yes                | `cargo fmt --check` |
| Lint       | yes                | `cargo clippy`     |
| Tests      | yes                | `cargo test`       |
| Build      | yes                | `cargo build`      |
| Security   | advisory only       | `cargo audit`      |

## Local setup

Install git hooks:

```bash
git config core.hooksPath .githooks
```

## Coverage

Coverage is collected with `tarpaulin` and reported in the CI summary.
Target: ≥70%.
