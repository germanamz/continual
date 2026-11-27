---
type: ticket
title: Join the guard to the registry
blocks: tickets/first-release/3-built-in-checks/judge-with-the-base-guard-settings
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: high
status: pending
---

# Join the guard to the registry

## Goal

Create the `continual-registry-guard` bridge and wire it, so a check step runs the guard with
its settings and the run's CI inputs.

## Design

[Built-in checks: guard](../../../docs/built-in-checks.md#guard) and
[bridges](../../../docs/built-in-checks.md#bridges).

## Behaviours to test

- The bridge states the name `guard` and that the check is a repository check.
- `protected` is read into the guard's settings, with its default, and an unknown key,
  `approval_label` among them, or a value of the wrong type is rejected with its line.
- The run's base revision, and whether it is a CI run and with which labels, reach the guard.
- The bridge passes the registry's `Check` contract suite.
- The real binary runs a `guard` check step on a fixture repository.

## Done when

- The composition root builds the guard core and the bridge, and hands the bridge to the
  registry.
- The bridge depends on the registry core and the guard core and on no other core or adapter,
  and `arch.toml` classifies it.
