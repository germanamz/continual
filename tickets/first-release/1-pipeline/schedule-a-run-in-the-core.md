---
type: ticket
title: Schedule a run in the core
blocks: tickets/first-release/1-pipeline/drive-a-run-through-the-tool-runner
parent: tickets/first-release/1-pipeline/story
points: 3
priority: high
status: pending
---

# Schedule a run in the core

## Goal

Give the core the `pipeline` module's `Run`: a pure state machine that plans the steps asked
for and everything upstream, says which steps are ready, records each outcome, skips the
downstream of a failed required step, and ends with a verdict. No port is involved yet.

## Design

The `pipeline` module in [Pipeline: core](../../../docs/pipeline.md#core),
[the graph](../../../docs/pipeline.md#the-graph) and the exit-code table in
[output and exit codes](../../../docs/pipeline.md#output-and-exit-codes).

## Behaviours to test

- A run includes the steps asked for and every step upstream of them, and nothing else.
- A step is ready only once every step it depends on has finished.
- A failed required step skips every step downstream of it, transitively.
- A failed step with `required = false` is reported, and its dependents still become ready.
- A timed-out or killed tool counts as failed; a missing program counts as could-not-run.
- A group step passes once its dependencies finish.
- The verdict names every failed step in the order they failed and every skipped step, and
  maps to exit code 0, 1 or 2 as the design's table says.

## Done when

- The state machine is tested through the core's public API, with no port and no I/O.
