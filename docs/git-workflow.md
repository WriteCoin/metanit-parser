# Git workflow

We use **Git Flow**.

## Why Git Flow

This project has versioned releases and a clear separation between active
development and stable releases — the exact scenario Git Flow was designed for.

## Branches

| Branch       | Purpose                         | Protected |
| ------------ | ------------------------------- | --------- |
| `main`       | Latest stable release           | Yes       |
| `develop`    | Integration branch for features | Yes       |
| `feature/*`  | New features                    | —         |
| `release/*`  | Release preparation             | —         |
| `hotfix/*`   | Urgent fixes on main            | —         |

## Rules

1. No direct pushes to `main` or `develop`. All changes merge via pull request.
2. Feature branches branch from `develop` and merge back into `develop`.
3. Release branches branch from `develop` and merge into both `main` and `develop`.
4. Hotfix branches branch from `main` and merge into both `main` and `develop`.
5. Every merge requires CI passing and at least one code review.

## Release process

1. Create `release/vX.Y.Z` from `develop`.
2. Bump version, update changelog, run final tests.
3. Merge into `main` and tag `vX.Y.Z`.
4. Merge back into `develop`.

## Risks

- Merge overhead from maintaining two long-lived branches.
- Release branches can live too long and diverge from develop.
