---
type: ticket
title: Join the architecture check to the registry
parent: tickets/first-release/3-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Join the architecture check to the registry

## Goal

Create the `continual-registry-architecture` bridge and wire it, so a check step runs the
architecture check with the layers its settings declare.

## Design

[Built-in checks: architecture](../../../docs/built-in-checks.md#architecture) and
[bridges](../../../docs/built-in-checks.md#bridges).

## Carried over

- `interfaces` is a layer key too
  ([Built-in checks: architecture](../../../docs/built-in-checks.md#architecture)), and the
  bridge reads it with the others.

## Behaviours to test

- The bridge states the name `architecture` and that the check is a repository check.
- `layers`, each with its `name`, `units`, `may_depend_on`, `independent`, `couplings`,
  `external` and `forbidden_imports`, are read into the architecture core's settings.
- An unknown key and a value of the wrong type are rejected with their lines, and so is a
  setting the core rejects, such as a coupling or an allow-list entry without a reason.
- The bridge passes the registry's `Check` contract suite.
- The real binary runs an `architecture` check step on a fixture repository.

## Done when

- The composition root builds the architecture core and the bridge, and hands the bridge to
  the registry.
- The bridge depends on the registry core and the architecture core and on no other core or
  adapter, and `arch.toml` classifies it.
