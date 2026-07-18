# Contributing

## Quick start

```bash
# 1. Clone the repository
git clone git@github.com:WriteCoin/metanit-parser.git
cd metanit-parser

# 2. Install git hooks
git config core.hooksPath .githooks

# 3. Create a feature branch from develop
git checkout develop
git pull github develop
git checkout -b feature/my-feature develop

# 4. Make changes and commit
git add -A
git commit -m "feat: add my feature"

# 5. Push and create a Pull Request
git push github feature/my-feature
gh pr create --base develop --head feature/my-feature \
  --title "feat: add my feature" \
  --body "Closes #ISSUE_NUMBER"
```

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
