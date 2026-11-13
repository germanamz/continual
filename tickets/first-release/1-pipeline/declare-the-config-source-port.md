---
type: ticket
title: Declare the ConfigSource port
blocks:
  - tickets/first-release/1-pipeline/read-continual-toml
  - tickets/first-release/1-pipeline/check-the-pinned-version
parent: tickets/first-release/1-pipeline/story
points: 2
priority: high
status: pending
---

# Declare the ConfigSource port

## Goal

Add the `ConfigSource` port and the use case that loads a `Workspace` through it, turning the
configuration input into the validated step graph and reporting every problem at its location.

## Design

The `ConfigSource` row in [Pipeline: ports and adapters](../../../docs/pipeline.md#ports-and-adapters)
and the validation rules in [Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml).

## Behaviours to test

- Configuration input from the source becomes a `Workspace`, with defaults applied.
- Every graph problem the `Workspace` rejects is reported with the location the input gave it.
- A source that finds no configuration is reported as such, so the run cannot start.
- `ConfigSource` is documented per P5, and the fake passes its contract suite.

## Done when

- The contract suite and the fake live in the core's `testing` module.
