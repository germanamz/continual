---
type: ticket
title: Report a run on the terminal
blocks: tickets/first-release/1-pipeline/run-pipelines-from-the-command-line
parent: tickets/first-release/1-pipeline/story
points: 2
priority: high
status: pending
---

# Report a run on the terminal

## Goal

Implement `Report` in the `continual-terminal` adapter: pass every output line through as it
arrives, say why each failed step failed, and end with the verdict, on the outputs it is given.

## Design

[Pipeline: output and exit codes](../../../docs/pipeline.md#output-and-exit-codes) and the
`terminal` notes in [ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- The `Report` contract suite passes against the adapter.
- A tool's standard output lines go to standard output and its standard error lines to
  standard error, as they arrive and otherwise unchanged.
- In a run of more than one entry, each line carries the step's name as a prefix, such as
  `test | `, and its package in a monorepo; a run of one entry passes lines through bare.
- A failed step prints one line with its reason, such as `test failed: exit code 101`.
- The last line names every failed and skipped entry.
- Given buffers instead of standard output and error, the adapter writes the same text into
  them.

## Done when

- `arch.toml` classifies the new crate.
