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

Add the `ToolRunner` port and the use case that drives a `Run` through it as futures: one
future for each running tool entry, up to the job limit, each handing over its output lines
as they are written and ending with its outcome, until nothing is running or ready, stopping
every tool when asked.

## Design

The use-case loop in [Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core) and the `ToolRunner` row in
[ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- Independent ready entries start together, never more than the job limit at once across
  every pipeline of the run; when there are more than free jobs, they start in the order the
  `Run` gives them.
- With a job limit of 1, entries run one after another in that order.
- Two paths of one pipeline advance on their own: a slow entry on one does not hold up the
  entries of the other.
- Each step's command, directory and time limit reach the runner as the configuration gives
  them.
- Output lines are taken as the runner hands them back, each with its entry and stream, while
  other tools keep running.
- A completion with a timed-out, killed or missing outcome is recorded as the `Run` defines.
- Asking the run to stop stops every running tool, marks every entry not started as skipped,
  and gives exit code 130.
- `ToolRunner` is documented per P5, and the fake passes its contract suite.

## Done when

- The contract suite and the fake live in the engine core's `testing` module.
- The use case is tested with the fake alone.
- The engine core uses the language's futures and gains no dependency for them (B1).
