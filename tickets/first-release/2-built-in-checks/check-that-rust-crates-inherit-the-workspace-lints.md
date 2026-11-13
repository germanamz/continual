---
type: ticket
title: Check that Rust crates inherit the workspace lints
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Check that Rust crates inherit the workspace lints

## Goal

Add the first language-specific check, `rust-workspace-lints`: every crate sets
`[lints] workspace = true`, as the `xtask` arch check requires today, run in a step of its own.

## Design

[Built-in checks: language-specific checks](../../../docs/built-in-checks.md#language-specific-checks)
and its decision in [decisions](../../../docs/built-in-checks.md#decisions).

## Behaviours to test

- A crate whose manifest has no `[lints]` table, or one that is not `workspace = true`, is
  reported at its manifest.
- A crate that inherits the workspace lints passes.
- The rule is core code over a manifest fact only the `rust` adapter reports, and the `rust`
  adapter passes the `LanguageSyntax` contract suite for that fact.
- Each case the `xtask` arch tests cover for workspace lints gives the same findings.

## Done when

- Existing: the `xtask` arch check keeps passing its tests unchanged.
