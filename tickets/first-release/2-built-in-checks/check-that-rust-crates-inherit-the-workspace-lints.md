---
type: ticket
title: Check that Rust crates inherit the workspace lints
blocks: tickets/first-release/2-built-in-checks/join-the-rust-workspace-lints-check-to-the-registry
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Check that Rust crates inherit the workspace lints

## Goal

Create the core of the first language-specific check, `continual-rust-workspace-lints`: every
crate sets `[lints] workspace = true`, as the `xtask` arch check requires today.

## Design

[Built-in checks: language-specific checks](../../../docs/built-in-checks.md#language-specific-checks)
and its decision in [decisions](../../../docs/built-in-checks.md#decisions).

## Behaviours to test

- A crate whose manifest has no `[lints]` table, or one that is not `workspace = true`, is
  reported at its manifest.
- A crate that inherits the workspace lints passes.
- The rule reads a manifest fact through the core's own `LanguageSyntax`, which the fake and
  the `rust` adapter both pass the contract suite for.
- A file that is not a crate's manifest is answered as not read.
- Each case the `xtask` arch tests cover for workspace lints gives the same findings.

## Done when

- Existing: the `xtask` arch check keeps passing its tests unchanged.
- `arch.toml` classifies the new core, which depends on no other workspace crate and carries
  a `clippy.toml` of its own.
