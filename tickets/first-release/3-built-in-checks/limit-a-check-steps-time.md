---
type: ticket
title: Limit a check step's time
blocks: tickets/first-release/3-built-in-checks/match-patterns-in-the-execution-context
parent: tickets/first-release/3-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Limit a check step's time

## Goal

Give a check step the time limit every tool step has: the engine races the step's checks
against its `timeout` through its new `Timer` port, which the new `continual-context`
adapter, the execution context, answers. A check that hangs then fails its step as timed out
instead of holding up a run.

## Design

[Pipeline: timeouts](../../../docs/pipeline.md#timeouts), the `Timer` row in
[ports and adapters](../../../docs/pipeline.md#ports-and-adapters), the time-limit decision in
[decisions](../../../docs/pipeline.md#decisions), and the `context` note in
[Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters). The
crate is in [Workspace: planned crates](../../../docs/workspace.md#planned-crates).

## Behaviours to test

- A check step that ends within its timeout is unaffected, and one without a `timeout` takes
  the workspace default.
- A check step still running when its limit ends fails with `timed out after …`, counts as
  failed for `required`, and its future is dropped.
- The tools running beside a timed-out check step keep running, and the run goes on.
- `Timer` is documented per P5, and the fake and the adapter pass its contract suite.

## Done when

- The rule is tested in the engine core with the `Timer` and `CheckRunner` fakes, with no
  real waiting.
- `arch.toml` classifies the new crate, which depends on the engine core and on no other
  workspace crate, with any new dependency added per G8.
