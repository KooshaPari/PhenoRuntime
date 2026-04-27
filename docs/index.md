---
layout: home
title: PhenoRuntime
titleTemplate: Runtime substrate and adapter layer
hero:
  name: PhenoRuntime
  text: Runtime substrate and adapter layer
  tagline: Pluggable crates for LLM routing, MCP hosting, event transport, and persistence adapters.
  actions:
    - theme: brand
      text: Runtime Surface
      link: /phenoruntime/
    - theme: alt
      text: ADRs
      link: /adr/
features:
  - title: Workspace substrate
    details: Rust workspace with adapter crates and cross-language bindings for the Phenotype ecosystem.
  - title: Architecture-led
    details: ADRs and research notes describe the current shape and the intended direction.
  - title: Buildable docs
    details: The repo now has a VitePress entrypoint and Bun-driven docs commands.
---

## What it does

PhenoRuntime provides the runtime substrate other Phenotype projects depend on
for container execution, MCP hosting, scheduling, security hardening, and
developer experience tooling.

## What exists today

- Rust workspace root with adapter crates.
- Language bindings scaffolds for Go, Python, TypeScript, Mojo, and WASI.
- ADRs covering runtime architecture, security, scheduling, DX, and multi-cloud.
- Research notes for runtime and WebAssembly trends.

## Useful entrypoints

- [Runtime surface](/phenoruntime/)
- [ADRs](/adr/)
- [Research](/research/)
- [Worklogs](/worklogs/)
