---
type: ticket
title: Read Rust lint suppressions
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Read Rust lint suppressions

## Goal

Implement the suppressions operation of `LanguageSyntax` in `continual-rust`, so the guard's
lint rules work on Rust as the `xtask` guard does today.

## Design

The `rust` notes in [Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- The `LanguageSyntax` contract suite for suppressions passes against the adapter.
- Every `#[expect]` attribute is reported with its line.
- A change to `[workspace.lints]`, and a crate `[lints]` table that becomes anything but
  `workspace = true`, are reported as loosened lint configuration.

## Done when

- Existing: the `xtask` guard keeps passing its tests unchanged.
