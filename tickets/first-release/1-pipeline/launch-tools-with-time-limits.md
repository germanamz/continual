---
type: ticket
title: Launch tools with time limits
blocks: tickets/first-release/1-pipeline/stop-every-running-tool
parent: tickets/first-release/1-pipeline/story
points: 3
priority: high
status: pending
---

# Launch tools with time limits

## Goal

Make the `continual-process` adapter stop any tool that runs past its time limit, together
with every process it started, so a hung tool never hangs a run.

## Design

[Pipeline: timeouts](../../../docs/pipeline.md#timeouts) and the `process` notes in
[ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- The `ToolRunner` contract suite passes in full, time limits included.
- A tool that runs past its limit is asked to stop, then killed after 10 seconds, and is
  reported as timed out with the output it produced.
- A tool that starts child processes has its children stopped with it.
- A tool that finishes within its limit is unaffected.

## Done when

- The behaviours pass on Linux, macOS and Windows in CI.
- Any dependency for process groups or job objects is added per G8 with its reason.
