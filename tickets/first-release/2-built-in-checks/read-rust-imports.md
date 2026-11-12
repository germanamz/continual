---
type: ticket
title: Read Rust imports
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Read Rust imports

## Goal

Implement the imports operation of `LanguageSyntax` in `continual-rust` with `syn`, so
`forbidden-imports` works on Rust as the `xtask` arch check's source scan does today.

## Design

The `rust` notes in [Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- The `LanguageSyntax` contract suite for imports passes against the adapter.
- `use` items, grouped and renamed ones included, and full paths in expressions and types are
  reported with their lines.
- Each case the `xtask` arch source-scan tests cover gives the same findings.

## Done when

- The adapter uses `syn` from the workspace dependencies.
