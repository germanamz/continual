---
type: ticket
title: Check architecture in Continual
blocks:
  - tickets/first-release/3-built-in-checks/read-rust-crates-and-their-dependencies
  - tickets/first-release/3-built-in-checks/forbid-imports-in-a-layer
  - tickets/first-release/3-built-in-checks/load-units-from-the-source-tree
  - tickets/first-release/3-built-in-checks/forbid-interfaces-in-a-layer
parent: tickets/first-release/3-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Check architecture in Continual

## Goal

Create the architecture core, `continual-architecture`, with the layer rules over the units
and dependencies a language adapter reads from manifests, and its `LanguageSyntax` port with
the manifest operation.

## Design

[Built-in checks: architecture](../../../docs/built-in-checks.md#architecture),
[the check cores](../../../docs/built-in-checks.md#the-check-cores) and the architecture rows in
[ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- A unit in no layer, and a configured unit that does not exist, are each reported.
- A dependency on a layer outside `may_depend_on` is reported, and one inside it passes.
- In an `independent` layer, a dependency between two units is reported unless a coupling
  with a reason declares it.
- With `external`, an external dependency off the allow-list is reported, and an entry
  without a reason is a configuration error; without `external`, external dependencies pass.
- A unit may depend on itself.
- The fake passes the `LanguageSyntax` contract suite for manifests.

## Done when

- The rules are tested in the architecture core with the fake.
- `arch.toml` classifies the new core, which depends on no other core or adapter and carries
  a `clippy.toml` of its own.
