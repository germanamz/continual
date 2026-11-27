---
type: ticket
title: Run a step's files in batches
parent: tickets/first-release/4-events/story
points: 3
priority: medium
status: pending
---

# Run a step's files in batches

## Goal

Let a step opt in to batches with `batch = true`: when its `{files}` do not fit one command
line, Continual runs its tool once for each batch that fits, so a formatter still runs on a
very large commit while every other step keeps one run of its tool.

## Design

The `batch` paragraph in [Pipeline: placeholders](../../../docs/pipeline.md#placeholders), the
`events` module in [Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core), the
`ToolRunner` row in [ports and adapters](../../../docs/pipeline.md#ports-and-adapters) and the
batch decision in [decisions](../../../docs/pipeline.md#decisions).

## Behaviours to test

- With `batch = true`, files that do not fit the limit are split into batches that each fit,
  in a stable order, and the tool runs once for each batch, one after another.
- Every batch runs even after one fails, and the step fails when any batch did.
- The step's timeout covers its batches together: a step that runs past it is stopped and
  fails as timed out, and the batches not started do not run.
- A step whose files fit runs its tool once, with or without `batch`.
- `batch` on a step whose `run` has no `{files}` is a configuration error at its line.
- `continual-toml-config` reads `batch`, and a value that is not a boolean is rejected with
  its line.
- The `ToolRunner` contract suite covers a sequence of commands under one time limit, and
  the fake and `continual-process` pass it.

## Done when

- The rules are tested in the engine core with fakes, against a command-line limit the test
  gives.
