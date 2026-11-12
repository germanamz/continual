---
type: ticket
title: Validate pipelines in the core
blocks:
  - tickets/first-release/1-pipeline/schedule-a-run-in-the-core
  - tickets/first-release/1-pipeline/declare-the-config-source-port
parent: tickets/first-release/1-pipeline/story
points: 3
priority: high
status: pending
---

# Validate pipelines in the core

## Goal

Add pipelines to the core's `Workspace`: named graphs of entries, each a step with the entries
it depends on and whether it is required, and pipelines that depend on other pipelines, all
rejected at construction when a reference or a cycle is wrong.

## Design

The `workspace` module in [Pipeline: core](../../../docs/pipeline.md#core), the pipeline keys in
[Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml) and
[pipelines](../../../docs/pipeline.md#pipelines).

## Behaviours to test

- A pipeline name follows `CheckName`'s rules.
- An entry naming an unknown step is rejected, naming the pipeline and the step.
- An entry depending on an entry its pipeline does not list is rejected, naming both.
- A cycle among a pipeline's entries is rejected, naming them in order.
- A pipeline depending on an unknown pipeline, or a cycle among pipelines, is rejected.
- `required` defaults to `true` for every entry.
- The same step may appear in several pipelines with different dependencies.

## Done when

- Core tests cover each behaviour through the public API, with no port and no I/O.
