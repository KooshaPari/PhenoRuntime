# PhenoRuntime Docs Landing Surface

## Overview

Create a VitePress docs landing surface for the PhenoRuntime repository so the
existing README, ADRs, research notes, and worklog can be browsed from one
entrypoint.

## Goals

- Add a docs homepage that describes the current runtime scaffold.
- Expose routes for the existing ADRs, research notes, and worklog index.
- Make the docs build reproducibly with Bun.
- Keep the docs surface small and aligned with the existing repository content.

## Users

- Phenotype engineers reading runtime architecture notes.
- Agents tracing current implementation state before making changes.
- Maintainers looking for the repository's current runtime and roadmap material.

## Scenarios

1. A user opens the docs root and sees the current state of PhenoRuntime.
2. A user navigates to architecture pages from the docs homepage.
3. A user builds the docs locally with Bun and gets a passing result.

## Acceptance Criteria

- `docs/` builds successfully with `bun run docs:build`.
- The docs homepage links to the runtime ADRs, research notes, and worklog.
- The ADR-backed routes resolve without dead-link failures during the build.
- `README.md` points readers to the docs surface.

## Out of Scope

- Implementing the runtime itself.
- Publishing a hosted deployment.
- Editing the ADR content beyond route support.
