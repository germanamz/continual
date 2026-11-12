---
type: ticket
title: Validate step definitions in the core
blocks: tickets/first-release/1-pipeline/validate-pipelines
parent: tickets/first-release/1-pipeline/story
points: 2
priority: high
status: pending
---

# Validate step definitions in the core

## Goal

Give the core the `workspace` module's first slice: named step definitions, each a tool step
or a check step with its timeout, rejected at construction when a name or a shape is wrong.
Pipelines build on them next.

## Design

The `workspace` module in [Pipeline: core](../../../docs/pipeline.md#core), and the step keys in
[Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml).

## Behaviours to test

- A step name follows `CheckName`'s rules, and an invalid one is rejected with the first
  offending character.
- A step with both `run` and `checks`, or with neither, is rejected.
- A timeout that is zero is rejected; a step without one takes the workspace default, which
  is 30 minutes unless set.
- Every problem carries the location the configuration input gave it, so it can be printed
  as `continual.toml:line: message`.

## Done when

- Core tests in `crates/continual-core/tests/` cover each behaviour through the public API.
- No port, adapter or I/O is added.
