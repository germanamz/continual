---
type: ticket
title: Join the rust-workspace-lints check to the registry
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Join the rust-workspace-lints check to the registry

## Goal

Create the `continual-registry-rust-workspace-lints` bridge and wire it, so a repository runs
the first language-specific check in a step of its own.

## Design

[Built-in checks: language-specific checks](../../../docs/built-in-checks.md#language-specific-checks)
and [bridges](../../../docs/built-in-checks.md#bridges).

## Behaviours to test

- The bridge states the name `rust-workspace-lints` and that the check is a per-file check.
- The check has no settings, so any key beside its name is rejected with its line.
- A manifest's path and text reach the core, each finding comes back at its manifest, and a
  file that is not a crate's manifest is answered as not read.
- The bridge passes the registry's `Check` contract suite.
- The real binary runs a `rust-workspace-lints` check step on a fixture repository.

## Done when

- The composition root builds the core and the bridge, and hands the bridge to the registry.
- The bridge depends on the registry core and the `rust-workspace-lints` core and on no other
  core or adapter, and `arch.toml` classifies it.
