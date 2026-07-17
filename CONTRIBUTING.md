# Contributing

## Branching model

We use Git Flow. See [docs/git-workflow.md](docs/git-workflow.md).

## PR checklist

Before opening a pull request:

- [ ] `cargo fmt` — no formatting diff
- [ ] `cargo clippy` — no warnings
- [ ] `cargo test` — all tests pass
- [ ] New code has tests where practical
- [ ] Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/)

## Commit message format

```
<type>: <short description>

Types: feat, fix, docs, test, ci, refactor, chore
```

## Code review

All changes must go through a pull request review. Direct pushes to `main` and
`develop` are forbidden. Every PR needs at least one approval before merge.
