---
type: ticket
title: Validate step definitions in the engine
blocks: tickets/first-release/1-pipeline/validate-pipelines
parent: tickets/first-release/1-pipeline/story
points: 2
priority: high
status: pending
---

# Validate step definitions in the engine

## Goal

Create the engine core, `continual-engine`, with the `workspace` module's first slice: named
step definitions, each a tool step or a check step with its timeout, rejected at construction
when a name or a shape is wrong. Pipelines build on them next.

## Design

The `workspace` module in [Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core), and the step keys in
[Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml). The crate is in
[Workspace: planned crates](../../../docs/workspace.md#planned-crates).

## Behaviours to test

- A step name follows the rules a check name follows, in a type of the engine's own, and an
  invalid one is rejected with the first offending character.
- A step with both `run` and `checks`, or with neither, is rejected.
- A timeout that is zero is rejected; a step without one takes the workspace default, which
  is 30 minutes unless set.
- A job limit below 1 is rejected; a workspace without one leaves the limit to its caller.
- Every problem carries the location the configuration input gave it, so it can be printed
  as `continual.toml:line: message`.

## Done when

- Tests in the engine core's `tests/` directory cover each behaviour through the public API.
- `arch.toml` classifies the new core, which depends on no other workspace crate and carries
  a `clippy.toml` of its own.
- No port, adapter or I/O is added.
