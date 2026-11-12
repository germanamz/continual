---
type: ticket
title: Check comments in every tracked file
blocks: tickets/first-release/2-built-in-checks/run-check-steps
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: high
status: pending
---

# Check comments in every tracked file

## Goal

Add the `SourceTree` port and its `continual-git` implementation, and the core use case that
checks comments in every file git tracks or would track, choosing each file's language adapter
by its extension.

## Design

The `SourceTree` row and the `git` notes in
[Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters), and
[check steps](../../../docs/built-in-checks.md#check-steps).

## Behaviours to test

- Every tracked file and every untracked file that is not ignored is listed; ignored files are
  not.
- Each file is checked by the language adapter that states its extension; a file no adapter
  handles is not inspected.
- With no file in a supported language, the check fails instead of passing.
- A file that disappears between listing and reading is reported, not skipped silently.
- The `SourceTree` contract suite passes against the fake and the `continual-git` adapter.

## Done when

- `continual-git` is created here if it does not exist yet, and `arch.toml` classifies it.
