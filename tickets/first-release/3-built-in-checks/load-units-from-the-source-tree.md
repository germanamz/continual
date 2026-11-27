---
type: ticket
title: Load units from the source tree
blocks: tickets/first-release/3-built-in-checks/join-the-architecture-check-to-the-registry
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Load units from the source tree

## Goal

Give the architecture core its loading: a use case that finds the manifests and source files
under a package through the core's own `SourceTree` port, reads each with its language
adapter and hands the units, their dependencies and their imports to the rules.

## Design

The loading bullet in
[Built-in checks: the check cores](../../../docs/built-in-checks.md#the-check-cores) and the
architecture rows in [ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Carried over

- A layer can forbid interfaces
  ([Forbid interfaces in a layer](forbid-interfaces-in-a-layer.md)), so the loading also hands
  the rules the interfaces each source file declares.

## Behaviours to test

- Every manifest a language adapter states it handles becomes a unit, and the source files
  under a unit's directory are that unit's.
- A configured unit with no manifest is reported, so the check cannot pass vacuously.
- A file that cannot be read makes the check could-not-run, never "no findings".
- The architecture core's `SourceTree` is documented per P5, and its contract suite passes
  against the fake and the `continual-git` adapter.

## Done when

- The use case is tested in the architecture core with fakes, and the rules still take facts
  and call no port (M5).
