---
type: ticket
title: Launch tools in parallel
blocks:
  - tickets/first-release/1-pipeline/launch-tools-with-time-limits
  - tickets/first-release/1-pipeline/run-steps-from-the-command-line
parent: tickets/first-release/1-pipeline/story
points: 3
priority: high
status: pending
---

# Launch tools in parallel

## Goal

Implement `ToolRunner` in the `continual-process` adapter: launch several tools at once and
hand back each one's outcome and captured output as it finishes.

## Design

[Pipeline: commands](../../../docs/pipeline.md#commands) and the `process` notes in
[ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- The `ToolRunner` contract suite passes against the adapter, apart from time limits.
- A tool runs in the directory it is given, with Continual's environment, and its standard
  output and error are captured together, byte for byte, including output that is not UTF-8.
- A tool's standard input is closed, so a tool that reads it ends instead of waiting.
- A program that is not on the `PATH` is reported as missing, not as a failed run; on Windows
  the lookup finds `.exe` and the other `PATHEXT` extensions.
- Several tools run at the same time, and each completion is handed back as it happens.

## Done when

- The behaviours pass on Linux, macOS and Windows in CI, and `arch.toml` classifies the crate.
