---
type: ticket
title: Check several cores in the arch check
parent: tickets/first-release/0-cores/story
points: 3
priority: high
status: pending
---

# Check several cores in the arch check

## Goal

Teach `cargo xtask arch` that a workspace holds several cores: `arch.toml` names the directory
that holds them, each core has an allow-list of its own, and no core depends on another, so
the next core arrives with its boundary checked.

## Design

The planned `arch` paragraph in
[Checks: `cargo xtask` checks](../../../docs/checks.md#cargo-xtask-checks), and rules B1, H1 and
H4 in [CONVENTIONS.md](../../../CONVENTIONS.md#cores-h).

## Behaviours to test

- Every crate under the cores directory is a core, and a directory that holds none fails the
  check, so it cannot pass vacuously.
- A core that depends on another core, or on any other workspace crate, is reported.
- A core's external dependency off its own allow-list is reported, and an entry on one core's
  list does not allow the crate in another core.
- An adapter that depends on two cores passes; one that depends on another adapter without a
  declared coupling is still reported.
- The source scan reports a forbidden `std` path in any core.
- `cargo xtask coverage` requires 95% of the lines of each core.
- Existing: every other arch rule keeps its findings on the existing fixtures.

## Done when

- `arch.toml` names `crates/cores` and holds the registry core's empty allow-list.
- docs/checks.md describes the check as built, and its planned marker is gone.
- `CONVENTIONS.md` and `AGENTS.md` describe the cores directory as built.
