---
type: ticket
title: Report a run through the Report port
blocks: tickets/first-release/1-pipeline/report-a-run-on-the-terminal
parent: tickets/first-release/1-pipeline/story
points: 2
priority: high
status: pending
---

# Report a run through the Report port

## Goal

Add the `Report` port and have the run use case pass every output line to it as the line
arrives, tell it why each failed entry failed, and give it the verdict at the end.

## Design

The `Report` row in [Pipeline: ports and adapters](../../../docs/pipeline.md#ports-and-adapters)
and [output and exit codes](../../../docs/pipeline.md#output-and-exit-codes).

## Behaviours to test

- Each output line reaches `Report` as it arrives, with its entry, package and stream, before
  the tool finishes.
- A failed entry is reported with its reason: the tool's exit code, its timeout, or why it
  could not run.
- A skipped entry is reported as skipped, with the failure that caused it.
- The verdict is reported once, last.
- `Report` is documented per P5, and the fake passes its contract suite.

## Done when

- The contract suite and the fake live in the core's `testing` module.
