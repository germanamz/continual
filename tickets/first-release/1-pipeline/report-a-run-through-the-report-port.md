---
type: ticket
title: Report a run through the Report port
blocks: tickets/first-release/1-pipeline/report-a-run-on-the-terminal
parent: tickets/first-release/1-pipeline/story
points: 2
priority: high
status: pending
---

# Report a run through the Report port

## Goal

Add the `Report` port and have the run use case tell it when each step starts, when each
finishes with its outcome and captured output, and the verdict at the end.

## Design

The `Report` row in [Pipeline: ports and adapters](../../../docs/pipeline.md#ports-and-adapters) and
[output and exit codes](../../../docs/pipeline.md#output-and-exit-codes).

## Behaviours to test

- `Report` hears every step start and finish, in the order they happen, with each step's
  package, outcome and output.
- A skipped step is reported as skipped, with the failed step that caused it.
- The verdict is reported once, last.
- `Report` is documented per P5, and the fake passes its contract suite.

## Done when

- The contract suite and the fake live in the core's `testing` module.
