---
type: ticket
title: Read Rust crates and their dependencies
blocks: tickets/first-release/2-built-in-checks/read-rust-imports
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Read Rust crates and their dependencies

## Goal

Implement the manifest operation of `LanguageSyntax` in `continual-rust`: read each crate and
its dependencies from `Cargo.toml` text, so the architecture check works on Rust without
running `cargo metadata`.

## Design

The `rust` notes and the manifest decision in
[Built-in checks](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- The `LanguageSyntax` contract suite for manifests passes against the adapter.
- Normal, dev and build dependencies are read, target-specific ones included.
- A dependency inherited with `workspace = true` resolves through the workspace manifest.
- A path dependency is told apart from an external one.
- This repository's `arch.toml` rules, written as the check's settings, report the same
  violations as `cargo xtask arch` on its fixtures.

## Done when

- Existing: the `xtask` arch check keeps passing its tests unchanged.
