---
type: ticket
title: Map events to pipelines and fill placeholders
blocks: tickets/first-release/3-events/pass-an-events-files-to-its-steps
parent: tickets/first-release/3-events/story
points: 2
priority: medium
status: pending
---

# Map events to pipelines and fill placeholders

## Goal

Give the core the `events` module's first slice: read `[events]`, pick the pipelines an event
runs with the pipelines they depend on, and refuse a run whose steps use a placeholder it
cannot fill.

## Design

[Pipeline: placeholders](../../../docs/pipeline.md#placeholders),
[events](../../../docs/pipeline.md#events) and the `events` module in
[Pipeline: core](../../../docs/pipeline.md#core).

## Behaviours to test

- An event runs the pipelines `[events]` maps to it and the pipelines they depend on.
- An unknown event name, or an unknown pipeline in `[events]`, is a configuration error.
- A step whose `run` uses a placeholder the run does not provide is a configuration error for
  that run, naming the step and the placeholder.
- `{message_file}` is provided only by `on_commit_msg`, and becomes the message file's path.

## Done when

- The rules are tested in the core with fakes.
