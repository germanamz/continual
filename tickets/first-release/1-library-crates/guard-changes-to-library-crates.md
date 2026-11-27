---
type: ticket
title: Guard changes to library crates
blocks: tickets/first-release/1-library-crates/share-locations-in-the-first-library-crate
parent: tickets/first-release/1-library-crates/story
points: 1
priority: high
status: pending
---

# Guard changes to library crates

## Goal

Make every change under `crates/libraries/` a guarded change in `cargo xtask guard`, so what
the cores share changes only with a contributor's approval (H8).

## Design

The `guard` bullet in [Checks: library crates](../../../docs/checks.md#library-crates), the
decision "Library crates are gates" in [decisions](../../../docs/checks.md#decisions), and rule
H8 in [CONVENTIONS.md](../../../CONVENTIONS.md#cores-h).

## Carried over

- Rule R2 in `CONVENTIONS.md` and the `guard` paragraph in `docs/checks.md` list what the
  guard reports. This task adds the library crates to both, and names the guard in H8's
  "Check" cell and in the guard row of the enforcement map; the `CONVENTIONS.md` edits are
  guarded changes.

## Behaviours to test

- A file added, changed or deleted under `crates/libraries/` is reported as a gate file.
- A file under `crates/cores/` or `crates/adapters/` is still reported only for what the
  guard reports there today.

## Done when

- The directory is in the list in `xtask/src/guard/protected.rs`.
- This task's planned marker is gone from `docs/checks.md`.
