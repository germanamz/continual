---
type: ticket
title: Stop every running tool
blocks: tickets/first-release/1-pipeline/stop-a-run-on-ctrl-c
parent: tickets/first-release/1-pipeline/story
points: 2
priority: high
status: pending
---

# Stop every running tool

## Goal

Let the `continual-process` adapter stop every tool it is running, each with its children, on
request, so an interrupted run leaves nothing behind.

## Design

The Ctrl-C bullet in [Pipeline: timeouts](../../../docs/pipeline.md#timeouts) and the `process`
notes in [ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- A stop request asks every running tool to stop and kills what is left after 10 seconds.
- Each stopped tool's children stop with it.
- Each stopped tool is handed back as killed.

## Done when

- The `ToolRunner` contract suite covers the stop request, and the adapter passes it on every
  platform in CI.
