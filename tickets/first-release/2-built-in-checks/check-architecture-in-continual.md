---
type: ticket
title: Check architecture in Continual
blocks:
  - tickets/first-release/2-built-in-checks/read-rust-crates-and-their-dependencies
  - tickets/first-release/2-built-in-checks/forbid-imports-in-a-layer
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Check architecture in Continual

## Goal

Write the architecture check's layer rules in the core, over the units and dependencies a
language adapter reads from manifests, with the manifest operation added to `LanguageSyntax`.

## Design

[Built-in checks: architecture](../../../docs/built-in-checks.md#architecture) and the
`LanguageSyntax` row in [ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- A unit in no layer, and a configured unit that does not exist, are each reported.
- A dependency on a layer outside `may-depend-on` is reported, and one inside it passes.
- In an `independent` layer, a dependency between two units is reported unless a coupling
  with a reason declares it.
- With `external`, an external dependency off the allow-list is reported, and an entry
  without a reason is a configuration error; without `external`, external dependencies pass.
- A unit may depend on itself.
- The fake passes the `LanguageSyntax` contract suite for manifests.

## Done when

- The rules are tested in the core with the fake.
