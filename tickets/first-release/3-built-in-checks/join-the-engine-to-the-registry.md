---
type: ticket
title: Join the engine to the registry
blocks: tickets/first-release/3-built-in-checks/run-check-steps
parent: tickets/first-release/3-built-in-checks/story
points: 3
priority: high
status: pending
---

# Join the engine to the registry

## Goal

Declare the engine's `CheckRunner` port and create the `continual-engine-registry` bridge,
which answers it by calling the check registry, so the engine can reach checks it knows
nothing of.

## Design

[Built-in checks: bridges](../../../docs/built-in-checks.md#bridges), the `CheckRunner` row in
[Pipeline: ports and adapters](../../../docs/pipeline.md#ports-and-adapters) and the run's inputs
in [Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core).

## Behaviours to test

- `CheckRunner` is documented per P5, and the fake passes its contract suite.
- The bridge passes the same suite over a registry of fake checks.
- A check step's entries, package, files and the run's inputs reach the registry, and each
  finding comes back as a `path:line: message` line with the step's outcome.
- A check or a setting the registry rejects comes back as a rejection with its line.
- Findings that only warn come back as lines with an outcome that does not fail the step.

## Done when

- The contract suite and the fake live in the engine core's `testing` module.
- The bridge depends on the engine core and the registry core and on no other core or
  adapter, and `arch.toml` classifies it.
- The engine does not call the port yet: [Run check steps](run-check-steps.md) does, and wires
  the bridge.
