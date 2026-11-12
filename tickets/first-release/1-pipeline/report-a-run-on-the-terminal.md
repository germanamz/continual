---
type: ticket
title: Report a run on the terminal
blocks: tickets/first-release/1-pipeline/run-steps-from-the-command-line
parent: tickets/first-release/1-pipeline/story
points: 2
priority: high
status: pending
---

# Report a run on the terminal

## Goal

Implement `Report` in the `continual-terminal` adapter, writing a run's progress, each step's
output and the verdict to the output it is given.

## Design

[Pipeline: output and exit codes](../../../docs/pipeline.md#output-and-exit-codes) and the
`terminal` notes in [ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- The `Report` contract suite passes against the adapter.
- A step's start prints one line naming the step and its package.
- A finished step prints its output whole under one line with its outcome, so two steps'
  output never interleaves.
- The last line names every failed and skipped step.
- Given a buffer instead of standard output, the adapter writes the same text into it.

## Done when

- `arch.toml` classifies the new crate.
