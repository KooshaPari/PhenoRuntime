# PhenoRuntime

[![License: MIT or Apache-2.0](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-yellow.svg)](LICENSE-MIT)
[![CI](https://github.com/KooshaPari/PhenoRuntime/actions/workflows/ci.yml/badge.svg)](https://github.com/KooshaPari/PhenoRuntime/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

Runtime substrate and adapter layer for the Phenotype ecosystem — pluggable crates providing LLM routing, MCP server hosting, event bus integration, and persistence adapters that bridge Phenotype agents with backing services.

**Part of the [Phenotype org](https://github.com/KooshaPari) ecosystem.** Shares governance, CI reusables, and conventions with [phenoShared](../phenoShared/) and the broader Phenotype polyrepo. Follows org standards: conventional commits, feature branching, Apache-2.0 + MIT dual licensing.

## What it does

PhenoRuntime is the Rust workspace that provides the pluggable adapters every Phenotype agent and service depends on at runtime. Instead of each project reimplementing LLM routing, MCP server plumbing, NATS event transport, or SurrealDB / MinIO adapters, they pull from this workspace.

The workspace is consumed as a git dependency (per-crate) by agent runtimes, MCP hosts, daemon processes, and the skill execution surface across the Phenotype polyrepo.

## Status

**Active.** Core crates (`phenotype-mcp-server`, `phenotype-llm`, `pheno-nats`, `pheno-minio`, `phenotype-surrealdb`) are wired into downstream consumers. See [CHANGELOG.md](./CHANGELOG.md) for release notes.

## Requirements

- Rust stable (1.80+ recommended, edition 2021)
- `cargo` workspace tooling
- For full integration tests: NATS server, SurrealDB, MinIO (optional — integration-gated)

## Quick start

```bash
# Build the whole workspace
cargo build --workspace

# Run unit tests
cargo test --workspace

# Lint with warnings as errors
cargo clippy --workspace --all-targets -- -D warnings

# Format check
cargo fmt --all -- --check
```

For docs work, use Bun:

```bash
bun install
bun run docs:build
```

Use an individual crate as a dependency:

```toml
[dependencies]
phenotype-mcp-server = { git = "https://github.com/KooshaPari/PhenoRuntime" }
phenotype-llm       = { git = "https://github.com/KooshaPari/PhenoRuntime" }
pheno-nats          = { git = "https://github.com/KooshaPari/PhenoRuntime" }
pheno-minio         = { git = "https://github.com/KooshaPari/PhenoRuntime" }
phenotype-surrealdb = { git = "https://github.com/KooshaPari/PhenoRuntime" }
```

## Structure

```
crates/
  phenotype-mcp-server/   # MCP server runtime — tool/resource/prompt registration
  phenotype-llm/          # LLM client adapters and routing primitives
  pheno-nats/             # NATS event bus adapter (publisher + subscriber wiring)
  pheno-minio/            # MinIO / S3-compatible object storage adapter
  phenotype-surrealdb/    # SurrealDB persistence adapter
src/                      # Workspace-level glue (binaries, integration harness)
docs/                     # Runtime design notes and architecture
ADR.md                    # Architecture decision records
PRD.md                    # Product requirements
PLAN.md                   # Roadmap and sequencing
CHARTER.md                # Scope and ownership
```

## Design principles

- **No inter-crate coupling within the workspace.** Each adapter stands alone and can be consumed independently.
- **Trait-first adapters.** Public surfaces are traits; implementations are behind feature flags where practical.
- **Fail loudly.** Required config failures are hard errors — no silent fallback to in-memory stubs in production builds.
- **Hexagonal-friendly.** Adapters slot behind ports defined in [phenoShared](https://github.com/KooshaPari/phenoShared)'s `phenotype-port-interfaces`.

## Technology Stack

- **Language**: Rust (edition 2021, stable 1.80+)
- **Async Runtime**: Tokio with multi-threaded scheduler
- **Serialization**: serde + serde_json for all data contracts
- **Error Handling**: thiserror for composable error types
- **Testing**: criterion for benchmarks, proptest for property testing
- **Workspace Management**: cargo workspace with unified dependency pinning
- **External Dependencies**: Minimal — pinned to latest stable versions

## Key Characteristics

- **Zero Inter-Crate Coupling**: Each adapter (MCP, LLM, NATS, MinIO, SurrealDB) is self-contained
- **Trait-First Design**: Public APIs are abstract traits; implementations are concrete structs
- **Production-Grade Error Handling**: No silent fallbacks; required config failures are hard errors
- **Hexagonal Alignment**: Adapters implement ports defined in phenoShared's `phenotype-port-interfaces`
- **Independent Consumability**: Use any single crate without pulling transitive dependencies
- **Integration Testing**: Optional feature flags gate integration tests requiring external services

## Related Phenotype Projects

- **[phenoShared](../phenoShared/)** — Port interfaces and shared infrastructure; PhenoRuntime implements ports
- **[agileplus-agents](../agileplus-agents/)** — Primary consumer of phenotype-llm and phenotype-mcp-server
- **[Sidekick](../Sidekick/)** — Agent framework; uses PhenoRuntime adapters
- **[PhenoObservability](../PhenoObservability/)** — Integration point for tracing and metrics

## Docs surface

The `docs/` tree now serves as the repository docs landing surface. It exposes
the runtime ADRs, research notes, and worklog index through VitePress.

## Governance & Contributing

- **CLAUDE.md** — Workspace conventions and contributor guidelines
- **AGENTS.md** — Agent operating contract and testing workflow
- **ADR.md** — Architecture decision records for major design choices
- **PRD.md** — Product requirements and roadmap
- **CONTRIBUTING.md** — Detailed contribution workflow
- **SECURITY.md** — Security vulnerability reporting
- **Changelog**: [CHANGELOG.md](CHANGELOG.md)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) and [AGENTS.md](./AGENTS.md) for the agent-driven workflow. Report security issues per [SECURITY.md](./SECURITY.md).

## License

Dual-licensed under Apache-2.0 OR MIT. See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).
