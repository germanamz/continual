---
type: ticket
title: Validate the step graph in the core
blocks:
  - tickets/first-release/1-pipeline/schedule-a-run-in-the-core
  - tickets/first-release/1-pipeline/declare-the-config-source-port
parent: tickets/first-release/1-pipeline/story
points: 3
priority: high
status: pending
---

# Validate the step graph in the core

## Goal

Give the core a `Workspace` value that is always a valid step graph: named tool, check and
group steps with their dependencies, `required` flags and timeouts, rejected at construction
when a reference, a cycle or a step's shape is wrong. Every later pipeline ticket builds on it.

## Design

The `workspace` module in [Pipeline: core](../../../docs/pipeline.md#core), and the keys in
[Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml).

## Behaviours to test

- A step name follows `CheckName`'s rules, and an invalid one is rejected with the first
  offending character.
- A `depends_on` entry naming an unknown step is rejected, naming both steps.
- A dependency cycle is rejected, naming its steps in order.
- A group step without `depends_on`, and a step with both a command and checks, are rejected.
- A timeout that is zero is rejected; a step without one takes the workspace default, which
  is 30 minutes unless set.
- `required` defaults to `true`.
- Every problem carries the location the configuration input gave it, so it can be printed
  as `continual.toml:line: message`.

## Done when

- Core tests in `crates/continual-core/tests/` cover each behaviour through the public API.
- No port, adapter or I/O is added.
