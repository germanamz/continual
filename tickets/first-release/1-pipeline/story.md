---
type: ticket
title: Run a repository's steps from continual.toml
blocks:
  - tickets/first-release/5-packages/story
  - tickets/first-release/3-events/story
  - tickets/first-release/2-built-in-checks/story
order: 1
parent: tickets/first-release/epic
priority: high
status: pending
---

# Run a repository's steps from continual.toml

## Goal

Let a repository declare its steps and its pipelines in a root `continual.toml` and run the
pipelines with `continual run`, each a graph whose required failures skip their downstream and
whose tools never run without a time limit, the same way on every machine and in CI
([pipeline](../../../VISION.md#pipeline), [tools](../../../VISION.md#tools)).

## Design

- [Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml),
  [commands](../../../docs/pipeline.md#commands), [timeouts](../../../docs/pipeline.md#timeouts),
  [pipelines](../../../docs/pipeline.md#pipelines) and
  [output and exit codes](../../../docs/pipeline.md#output-and-exit-codes).
- [Pipeline: core](../../../docs/pipeline.md#core),
  [ports and adapters](../../../docs/pipeline.md#ports-and-adapters) and
  [composition root](../../../docs/pipeline.md#composition-root).
- [Workspace: planned crates](../../../docs/workspace.md#planned-crates).

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk).

## Done when

- Every child ticket is completed.
- `continual run <pipeline>...` works on a single-package repository with tool steps and
  pipelines that depend on other pipelines.
- The design documents describe the change, and its planned markers are gone.
