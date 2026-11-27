---
type: ticket
title: Validate pipelines in the engine
blocks:
  - tickets/first-release/2-pipeline/schedule-a-run-in-the-engine
  - tickets/first-release/2-pipeline/declare-the-config-source-port
parent: tickets/first-release/2-pipeline/story
points: 3
priority: high
status: pending
---

# Validate pipelines in the engine

## Goal

Add pipelines to the engine's `Workspace`: named, ordered lists of entries that start at their
first entry, each a step with the entries above it that it depends on and whether it is
required, and pipelines that depend on other pipelines, all rejected at construction when an
entry, a reference or a cycle is wrong.

## Design

The `workspace` module in [Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core), the pipeline keys in
[Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml) and
[pipelines](../../../docs/pipeline.md#pipelines).

## Carried over

- The name rule is the library crate's `Name` (`crates/libraries/vocabulary/src/name.rs`),
  and the engine holds no copy of it. A pipeline name is a type of the engine's own built on
  `Name`, as `StepName` is in `crates/cores/engine/src/workspace/step_name.rs` (Q16).
- An invalid name is reported with the library crate's `NameError`, whose message names no
  kind of name, so the problem says it is a pipeline name, as
  `ProblemKind::InvalidStepName` says `invalid step name "Fmt": …` for a step. The engine
  does not re-export `NameError`: its tests name `continual_vocabulary::NameError`
  ([Workspace: open questions](../../../docs/workspace.md#open-questions)).
- `Workspace::build` reports `jobs`, the configuration's `timeout`, then the steps in the order
  of their names. Where the problems of pipelines come in that order is this task's to say,
  in the paragraph on configuration input in
  [Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core).

## Behaviours to test

- A pipeline name follows `CheckName`'s rules.
- An entry naming an unknown step is rejected, naming the pipeline and the step.
- A step listed twice in one pipeline is rejected.
- A starting entry with `depends_on`, and a later entry without it, are rejected.
- An entry depending on an entry that is not above it, or not in the pipeline, is rejected,
  naming both.
- Entries keep their written order, and a pipeline with no entries is valid.
- A pipeline depending on an unknown pipeline, or a cycle among pipelines, is rejected.
- `required` defaults to `true` for every entry.
- The same step may appear in several pipelines with different dependencies.

## Done when

- The engine core's tests cover each behaviour through the public API, with no port and no I/O.
