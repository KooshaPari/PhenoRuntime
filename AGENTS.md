# AGENTS.md — PhenoRuntime

## Project Overview

- **Name**: PhenoRuntime
- **Description**: Runtime environment and execution engine - multi-language bindings for the Phenotype runtime
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoRuntime`
- **Language Stack**: Rust (core), Go, Python, TypeScript, Mojo
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to PhenoRuntime
cd /Users/kooshapari/CodeProjects/Phenotype/repos/PhenoRuntime

# Rust core
cargo build --workspace
cargo test --workspace

# Go bindings
cd go && go build ./...

# Python bindings
cd python && pip install -e .

# TypeScript bindings
cd ts && npm install && npm run build

# Rust standalone
cd rust && cargo build
```

## Architecture

```
PhenoRuntime/
├── bindings/                  # Language bindings
├── Cargo.lock                # Rust dependencies
├── Cargo.toml                # Workspace manifest
├── crates/                   # Rust runtime crates
├── docs/                     # Documentation
├── examples/                 # Usage examples
├── ffi/                      # FFI layer
├── go/                       # Go SDK
│   └── go.mod
├── go.mod                    # Root Go module
├── mojo/                     # Mojo bindings
├── package.json              # Node dependencies
├── pyproject.toml            # Python config
├── python/                   # Python SDK
├── README.md                 # Overview
├── rust/                     # Rust standalone
├── src/                      # Rust source
└── ts/                       # TypeScript SDK
```

## Quality Standards

### Rust (Core)
- **Line length**: 100 characters
- **Formatter**: `cargo fmt`
- **Linter**: `cargo clippy -- -D warnings`
- **Tests**: `cargo test --workspace`

### Go Bindings
- **Line length**: 100 characters
- **Formatter**: `gofmt`, `goimports`
- **Linter**: `golangci-lint`
- **Tests**: `go test ./...`

### Python Bindings
- **Line length**: 100 characters
- **Formatter**: `ruff format`
- **Linter**: `ruff check`
- **Type checker**: `mypy`

### TypeScript Bindings
- **Line length**: 100 characters
- **Formatter**: `prettier`
- **Type checker**: `tsc --noEmit`

## Git Workflow

### Branch Naming
Format: `phenoruntime/<type>/<description>` or `<language>/<type>/<description>`

Examples:
- `phenoruntime/feat/wasm-executor`
- `go/feat/context-cancellation`
- `python/fix/memory-leak`

### Commit Format
```
<type>(<scope>): <description>

Scope: core, go, python, ts, mojo, ffi

Examples:
- feat(core): add WASM runtime support
- fix(go): resolve goroutine leak
- docs(python): update async examples
```

## File Structure

```
PhenoRuntime/
├── crates/                   # Rust runtime crates
├── bindings/                 # Language bindings
├── go/                       # Go SDK
├── python/                   # Python SDK
├── ts/                       # TypeScript SDK
├── mojo/                     # Mojo SDK
├── ffi/                      # FFI layer
└── rust/                     # Rust standalone
```

## CLI Commands

```bash
# Rust workspace
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Go
cd go && go build ./...
cd go && go test ./...

# Python
cd python && pip install -e ".[dev]"
cd python && pytest

# TypeScript
cd ts && npm install
cd ts && npm run build
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Rust build fails | Check workspace members in Cargo.toml |
| FFI linking errors | Verify binding definitions |
| Runtime panics | Check safety boundaries |

## Dependencies

- **PhenoProc/**: Related process management
- **nanovms/**: Potential deployment target
- **crates/phenotype-***: Related runtime crates

## Agent Notes

When working in PhenoRuntime:
1. Multi-language project with Rust core
2. FFI safety is critical - use `unsafe` minimally
3. Language bindings should match core semantics
4. Runtime changes affect all language SDKs
5. Test in all affected languages when changing core
