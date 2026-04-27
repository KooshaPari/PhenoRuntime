# Work Audit — PhenoRuntime

**Index:** See `docs/worklogs/README.md`.

## Purpose

This log records research completions, architectural decisions, issues discovered (duplication, performance, governance), work completions, and planning artifacts (fork candidates, migration plans) for PhenoRuntime.

## Worklog Categories

All entries MUST be organized into one of these categories:

| Category | File | Purpose |
|----------|------|---------|
| ARCHITECTURE | worklogs/ARCHITECTURE.md | ADRs, library extraction, major refactors |
| DUPLICATION | worklogs/DUPLICATION.md | Cross-project code duplication |
| DEPENDENCIES | worklogs/DEPENDENCIES.md | External deps, forks, modernization |
| INTEGRATION | worklogs/INTEGRATION.md | External integrations, bridge contracts |
| PERFORMANCE | worklogs/PERFORMANCE.md | Optimization, benchmarking, profiling |
| RESEARCH | worklogs/RESEARCH.md | Starred repo analysis, technology research |
| GOVERNANCE | worklogs/GOVERNANCE.md | Policy, evidence, quality gates, process |

## When to Write

Write an entry for:
- Research completions (starred repos, tech eval, design exploration)
- Significant decisions made (why we chose X over Y)
- Issues found (duplication >50 LOC, performance bottlenecks, governance gaps)
- Work completions (feature shipped, large refactor finished)
- Planning (fork candidates, migration roadmaps, deprecation notices)

## Format

Each entry should include:
- **Date** (YYYY-MM-DD)
- **Category** (from table above)
- **Title** (concise, searchable)
- **Context** (what prompted this work)
- **Finding/Decision** (what was discovered or decided)
- **Impact** (how it affects this project or others)
- **Project Tags** (e.g., `PhenoRuntime`, `[cross-repo]`)

## Example

```markdown
### 2026-04-24 | DUPLICATION | Config loader duplication in 5 projects

**Context:** Cross-project audit identified identical config loading patterns.
**Finding:** 50+ LOC duplicated in BytePort, Civis, Dino, Eidolon, FocalPoint.
**Decision:** Extract into phenotype-config-core shared crate.
**Impact:** -250 LOC across 5 repos; single source of truth for config.
**Tags:** `[cross-repo]` `[DUPLICATION]`
```

---

## 2026-04-27 | DEPENDENCIES | Cargo security refresh

**Context:** GitHub Dependabot reported stale Cargo.lock advisories for
`aws-sdk-s3`, `lru`, `jsonwebtoken`, and `rustls-webpki`.

**Finding:** `jsonwebtoken` was pulled only through an unused `surrealdb`
dependency in the scaffold wrapper. `aws-sdk-s3` and `lru` were fixed by moving
the MinIO adapter to the current AWS SDK line. The NATS adapter was also moved
from the incompatible `natsio` API to `async-nats`.

**Decision:** Remove unused `surrealdb`, update the AWS SDK/NATS manifests, and
keep the remaining `rustls-webpki 0.101.7` advisory tracked as an upstream AWS
Smithy legacy-rustls residual.

**Impact:** Workspace tests compile and pass again while the active dependency
graph is substantially reduced.

**Validation:**
- `cargo test --workspace --no-run`
- `cargo test --workspace`
- `cargo audit --no-fetch` (residual: `rustls-webpki 0.101.7` via
  `aws-smithy-http-client 1.1.12`)

**Tags:** `PhenoRuntime` `[DEPENDENCIES]` `[security]` `[cargo]`

---

Use parent Phenotype worklog surfaces only for aggregation tools and
cross-project analysis.
