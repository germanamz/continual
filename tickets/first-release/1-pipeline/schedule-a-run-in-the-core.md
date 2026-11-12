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

Give the core the `pipeline` module's `Run`: a pure state machine that plans the pipelines
asked for and the pipelines they depend on, says which entries are ready, records each outcome,
skips the downstream of a failed required entry and the pipelines that depend on a failed one,
and ends with a verdict. No port is involved yet.

## Design

The `pipeline` module in [Pipeline: core](../../../docs/pipeline.md#core),
[pipelines](../../../docs/pipeline.md#pipelines) and the exit-code table in
[output and exit codes](../../../docs/pipeline.md#output-and-exit-codes).

## Behaviours to test

- A run includes the pipelines asked for and every pipeline they depend on, and nothing else.
- A pipeline's entries become ready only once every pipeline it depends on has passed, and an
  entry only once every entry it depends on has finished.
- A failed required entry skips every entry downstream of it in its pipeline, transitively.
- A failed entry with `required = false` is reported, and its dependents still become ready.
- A failed pipeline skips every pipeline that depends on it.
- A step listed in two pipelines of the run runs once for each.
- A timed-out or killed tool counts as failed; a missing program counts as could-not-run.
- The verdict names every failed entry in the order they failed and every skipped entry, and
  maps to exit code 0, 1 or 2 as the design's table says.

## Done when

- The state machine is tested through the core's public API, with no port and no I/O.
