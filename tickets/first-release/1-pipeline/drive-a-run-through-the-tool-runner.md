---
type: ticket
title: Drive a run through the tool runner
blocks:
  - tickets/first-release/1-pipeline/report-a-run-through-the-report-port
  - tickets/first-release/1-pipeline/launch-tools-in-parallel
parent: tickets/first-release/1-pipeline/story
points: 3
priority: high
status: pending
---

# Drive a run through the tool runner

## Goal

Add the `ToolRunner` port and the use case that drives a `Run` through it: start every ready
tool step up to the job limit, wait for the next completion, record it, and repeat until
nothing is running or ready, stopping every tool when asked.

## Design

The use-case loop in [Pipeline: core](../../../docs/pipeline.md#core) and the `ToolRunner` row in
[ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- Independent ready steps start together, never more than the job limit at once.
- Each step's command, directory and time limit reach the runner as the configuration gives
  them.
- A completion with a timed-out, killed or missing outcome is recorded as the `Run` defines.
- Asking the run to stop stops every running tool, marks every step not started as skipped,
  and gives exit code 130.
- `ToolRunner` is documented per P5, and the fake passes its contract suite.

## Done when

- The contract suite and the fake live in the core's `testing` module.
- The loop is tested with the fake alone.
