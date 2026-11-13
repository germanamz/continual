---
type: ticket
title: Launch tools in parallel
blocks:
  - tickets/first-release/1-pipeline/launch-tools-with-time-limits
  - tickets/first-release/1-pipeline/run-pipelines-from-the-command-line
parent: tickets/first-release/1-pipeline/story
points: 3
priority: high
status: pending
---

# Launch tools in parallel

## Goal

Implement `ToolRunner` in the `continual-process` adapter: launch several tools at once and
hand back each line they write and each one's outcome as it happens.

## Design

[Pipeline: commands](../../../docs/pipeline.md#commands) and the `process` notes in
[ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- The `ToolRunner` contract suite passes against the adapter, apart from time limits.
- A tool runs in the directory it is given, with Continual's environment.
- Each line a tool writes is handed back as soon as it ends, byte for byte, with its stream:
  standard output and standard error stay apart, and output that is not UTF-8 is kept as is.
- A tool's standard input is closed, so a tool that reads it ends instead of waiting.
- A program that is not on the `PATH` is reported as missing, not as a failed run; on Windows
  the lookup finds `.exe` and the other `PATHEXT` extensions.
- Several tools run at the same time, and their lines and completions are handed back as they
  happen.

## Done when

- The behaviours pass on Linux, macOS and Windows in CI, and `arch.toml` classifies the crate.
