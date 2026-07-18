# Platform Comparison: GitHub Actions vs GitLab CI/CD

## Scenario

Configure CI/CD for the same Rust library project (`metanit-parser`) on both
GitHub Actions and GitLab CI/CD. Pipeline stages: lint, test, build, security.

## 1. Setup Complexity

| Metric | GitHub Actions | GitLab CI/CD |
|--------|---------------|--------------|
| Config files | `.github/workflows/ci.yml` (1 file) | `.gitlab-ci.yml` (1 file) |
| Lines of config | 56 (ci.yml) + 25 (codeql.yml) | 46 |
| Explicit steps | 5 jobs, 19 steps | 4 stages, 9 commands |
| Caching config | 3rd party action (`Swatinem/rust-cache@v2`) | Built-in `cache:` keyword |
| Artifacts | `actions/upload-artifact@v4` | Built-in `artifacts:` keyword |
| Runners | `ubuntu-latest` (managed by GitHub) | `image: rust:latest` (Docker) |

**Verdict:** GitLab CI/CD has slightly less boilerplate due to built-in caching
and artifacts. GitHub requires 3rd-party actions for equivalent functionality.

## 2. Code Review UX

| Aspect | GitHub Actions | GitLab CI/CD |
|--------|---------------|--------------|
| PR/MR creation | `gh pr create` | `glab mr create` |
| Review comments | Inline on PR, thread per comment | Similar inline, thread per comment |
| Required reviews | Branch protection rules | Merge request approvals |
| CI status in PR | Shown as check status | Shown as pipeline status |
| Fix after review | Push to same branch, PR updates | Push to same branch, MR updates |

**Verdict:** Nearly identical UX for code review. GitHub has slightly better
CLI tooling (`gh` is more mature than `glab`).

## 3. Pipeline Speed & Log Readability

| Aspect | GitHub Actions | GitLab CI/CD |
|--------|---------------|--------------|
| Cold start | ~15s for job setup | ~20s for Docker pull |
| Cache | 3rd-party, ~30s restore | Built-in, ~25s restore |
| Parallel jobs | Yes (free tier: 20 jobs) | Yes (free tier: 400 min/mo) |
| Log viewer | Web-based, collapsible sections | Web-based, collapsible sections |
| Search in logs | Browser find | Built-in search |
| Artifacts download | Via web UI or API | Via web UI or API |

**Verdict:** Comparable. GitHub Actions starts slightly faster; GitLab CI/CD
has better built-in cache performance.

## 4. Security Features

| Feature | GitHub Actions | GitLab CI/CD |
|---------|---------------|--------------|
| Protected branches | Yes (branch protection rules) | Yes (protected branches) |
| Required reviewers | Yes (in branch protection) | Yes (merge request approvals) |
| SAST (static analysis) | CodeQL (built-in) | SAST.gitlab-ci.yml (built-in) |
| Secret scanning | Yes (GitHub Advanced Security) | Yes (GitLab Secret Detection) |
| Dependency scanning | Dependabot + `cargo audit` | Dependency Scanning template |
| Container scanning | N/A (Rust project) | N/A (Rust project) |

**Verdict:** Both offer comprehensive security features. GitHub's CodeQL is
more mature for Rust; GitLab's security templates are more integrated.

## 5. Applicability

| Project type | Recommended platform | Reason |
|-------------|---------------------|--------|
| Educational | **GitHub** | Larger community, free for students, extensive learning resources |
| Research | **Either** | Both have free tiers for public repos; choose based on team preference |
| Industrial | **GitHub** | More mature marketplace, better third-party integrations |
| Open source | **GitHub** | De facto standard for open source, Discoverability |
| Enterprise with self-hosted | **GitLab** | Better self-hosted offering, built-in DevOps lifecycle |

## 6. Overall Comparison Table

| Criterion | GitHub Actions | GitLab CI/CD |
|-----------|---------------|--------------|
| Configuration effort | ★★★☆☆ (3rd-party deps) | ★★★★☆ (built-in) |
| Log readability | ★★★★☆ | ★★★★☆ |
| Speed | ★★★★☆ | ★★★☆☆ |
| Security (SAST) | ★★★★★ (CodeQL) | ★★★★☆ |
| Security (secrets) | ★★★★☆ | ★★★★☆ |
| Ease of review | ★★★★★ | ★★★★☆ |
| Caching | ★★★☆☆ (3rd party) | ★★★★★ (built-in) |
| Self-hosting | ★★☆☆☆ | ★★★★★ |
| Community | ★★★★★ | ★★★☆☆ |
| Documentation | ★★★★★ | ★★★★☆ |

## 7. Recommendation for This Project

For `metanit-parser` (an open-source educational Rust library):

**Recommended: GitHub Actions**

Reasons:
1. De facto standard for open-source Rust projects
2. Excellent integration with GitHub ecosystem
3. CodeQL provides mature Rust SAST analysis
4. Larger community means more reusable actions and troubleshooting resources
5. Free for public repositories with unlimited compute minutes

GitLab CI/CD would be preferred if:
- Self-hosting is required
- The project needs a complete DevOps lifecycle in one platform
- The team prefers GitLab's built-in caching and artifact management
