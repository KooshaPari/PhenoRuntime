# PhenoRuntime

Runtime substrate for the Phenotype ecosystem — daemon, skills, and the adapter crates that sit between Phenotype agents and their backing services (LLMs, MCP servers, object storage, event bus, and document store).

**Part of the [Phenotype org](https://github.com/KooshaPari) ecosystem.** Shares CI reusables and conventions with [phenoShared](https://github.com/KooshaPari/phenoShared) and the broader polyrepo. Follows org conventions: conventional commits, `<type>/<topic>` branching, Apache-2.0 + MIT dual license.

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

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) and [AGENTS.md](./AGENTS.md) for the agent-driven workflow. Report security issues per [SECURITY.md](./SECURITY.md).

## License

Dual-licensed under Apache-2.0 OR MIT. See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).
