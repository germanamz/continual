---
type: ticket
title: Pass an event's files to its steps
blocks:
  - tickets/first-release/3-events/read-an-events-files-from-change-history
  - tickets/first-release/5-packages/route-files-to-their-packages
parent: tickets/first-release/3-events/story
points: 3
priority: medium
status: pending
---

# Pass an event's files to its steps

## Goal

Expand `{files}` into one word per event file, filtered by each step's `files` globs, and skip
a step whose filtered files are empty, so a tool sees exactly the files the moment touched.

## Design

[Pipeline: placeholders](../../../docs/pipeline.md#placeholders) and the `events` module in
[Pipeline: core](../../../docs/pipeline.md#core).

## Behaviours to test

- `{files}` becomes one word per event file, relative to the step's package directory, in a
  stable order.
- `files` globs filter the event's files; a step without `files` takes them all.
- A file whose path holds spaces, non-ASCII characters or, on Windows, backslashes stays one
  word with its path intact.
- A step whose filtered files are empty is skipped and reported as skipped, not passed.

## Done when

- The rules are tested in the core with fakes.
- If matching globs needs a library in the core, it is a pure-computation crate added to
  `arch.toml`'s allow-list with a reason, after the contributor approves it.
