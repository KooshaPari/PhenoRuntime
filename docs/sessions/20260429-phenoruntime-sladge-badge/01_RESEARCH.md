# Research

## Repo Fit

PhenoRuntime is in scope for the sladge rollout because its README defines the
workspace as the runtime substrate for LLM routing, MCP server hosting, agent
runtimes, MCP hosts, daemon processes, and skill execution surfaces.

## Local State

Canonical `PhenoRuntime` had unrelated local `.agileplus/agileplus.db` state.
The badge change was prepared in an isolated worktree to avoid mixing that
state.

## Decision

Treat this as a documentation/governance badge update only. Do not modify Rust
crates, docs build configuration, or AgilePlus state.
