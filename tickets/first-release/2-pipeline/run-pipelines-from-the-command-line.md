---
type: ticket
title: Run pipelines from the command line
blocks: tickets/first-release/2-pipeline/stop-a-run-on-ctrl-c
parent: tickets/first-release/2-pipeline/story
points: 3
priority: high
status: pending
---

# Run pipelines from the command line

## Goal

Make `continual run <pipeline>... [--jobs <n>] [--unpinned]` work end to end on a
single-package repository, with the composition root wiring the config, process and terminal
adapters into the engine core.

## Design

[Pipeline: pipelines](../../../docs/pipeline.md#pipelines),
[output and exit codes](../../../docs/pipeline.md#output-and-exit-codes) and
[composition root](../../../docs/pipeline.md#composition-root).

## Behaviours to test

- Run against fixture repositories, the real binary runs the pipelines asked for and the
  pipelines they depend on, streams their output, and exits with 0, 1 or 2 as the design's
  table says.
- `continual run` with an unknown pipeline exits with code 2 and lists the pipelines that
  exist.
- The job limit is `[continual] jobs`, `--jobs` overrides it for one invocation, and without
  either it is the number of CPUs. A limit of 1 runs one entry at a time.
- A missing tool prints the step's `install` command, or says it has none.
- `--unpinned` runs a binary whose version is not the pinned one.
- Existing: `continual --help` and `--version` behave as before.

## Done when

- The composition root builds driven adapters, then the engine core, then the CLI (B5), and
  the runtime that executes the run's futures, with any new dependency added per G8.
