---
type: ticket
title: Move the core crate under crates/cores
blocks: tickets/first-release/0-cores/check-several-cores-in-the-arch-check
parent: tickets/first-release/0-cores/story
points: 2
priority: high
status: completed
---

# Move the core crate under crates/cores

## Goal

Move `crates/continual-core` to `crates/cores/registry` and name it `continual-registry`, so
the one core there is today sits where every core will, ready to become the check registry.

## Design

[Workspace: planned crates](../../../docs/workspace.md#planned-crates).

## Behaviours to test

- Existing: `CheckName` behaves as before, and its tests pass unchanged from their new place.
- Existing: `cargo xtask arch` and `cargo xtask coverage` hold the moved crate to the rules and
  the line coverage they held `continual-core` to.
- The crate's `clippy.toml` moves with it and still applies.

## Done when

- `arch.toml`, the checks' configuration and the agent rules name the crate by its new name
  and path.
- docs/workspace.md, `CONVENTIONS.md` and `AGENTS.md` describe the crate where it is.
- The guard's findings for the moved test files are reported to the contributor, since a move
  reads as tests deleted.
