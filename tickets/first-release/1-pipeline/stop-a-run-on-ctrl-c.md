---
type: ticket
title: Stop a run on Ctrl-C
parent: tickets/first-release/1-pipeline/story
points: 2
priority: high
status: pending
---

# Stop a run on Ctrl-C

## Goal

Turn Ctrl-C into a stop request in the composition root, so an interrupted `continual run`
stops every tool, reports what finished and exits with code 130.

## Design

The Ctrl-C bullet in [Pipeline: timeouts](../../../docs/pipeline.md#timeouts) and the `process`
notes in [ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- Interrupting the real binary during a run stops the running tools and their children.
- The steps not started are reported as skipped, the finished ones with their outcome.
- The exit code is 130.

## Done when

- Any dependency for signal handling is added per G8 with its reason.
- The design documents describe the change, and the planned markers of
  [Run a repository's steps from continual.toml](story.md) are gone
  once its other children are completed.
