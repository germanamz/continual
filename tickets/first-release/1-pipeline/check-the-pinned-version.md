---
type: ticket
title: Check the pinned version
blocks: tickets/first-release/1-pipeline/run-steps-from-the-command-line
parent: tickets/first-release/1-pipeline/story
points: 1
priority: high
status: pending
---

# Check the pinned version

## Goal

Make the workspace load refuse a binary whose version is not the one `[continual] version`
pins, unless the run is unpinned, so every machine runs the same release.

## Design

The `[continual] version` bullet in
[Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml).

## Behaviours to test

- A binary version that differs from the pin stops the load with a problem that names both
  versions and gives `cargo install continual --locked --version <pinned>`.
- An unpinned load runs with any version and says so.
- A matching version loads as before.

## Done when

- The binary's version is passed into the core as a value, never read there (X1, X4).
