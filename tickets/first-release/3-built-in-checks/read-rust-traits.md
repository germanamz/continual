---
type: ticket
title: Read Rust traits
parent: tickets/first-release/3-built-in-checks/story
points: 1
priority: medium
status: pending
---

# Read Rust traits

## Goal

Implement the interfaces operation of the architecture core's `LanguageSyntax` in
`continual-rust` with `syn`, so `interfaces = false` works on Rust, where an interface is a
trait.

## Design

The `rust` notes in
[Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Carried over

- If [Forbid traits in library crates](../1-library-crates/forbid-traits-in-library-crates.md)
  is merged, each case its `xtask` tests cover gives the same findings here.

## Behaviours to test

- The architecture core's `LanguageSyntax` contract suite for interfaces passes against the
  adapter.
- Each `trait` item is reported with its line, one inside an inline module included.
- An `impl` of a trait is not reported.

## Done when

- The adapter uses `syn` from the workspace dependencies.
