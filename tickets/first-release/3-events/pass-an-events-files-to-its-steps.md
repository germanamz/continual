---
type: ticket
title: Pass an event's files to its steps
blocks:
  - tickets/first-release/3-events/read-an-events-files-from-change-history
  - tickets/first-release/5-packages/route-files-to-their-packages
  - tickets/first-release/3-events/give-check-steps-an-events-files-and-message
  - tickets/first-release/3-events/run-a-steps-files-in-batches
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
[Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core).

## Behaviours to test

- `{files}` becomes one word per event file, relative to the step's package directory, in a
  stable order.
- `files` globs filter the event's files; a step without `files` takes them all.
- A file whose path holds spaces, non-ASCII characters or, on Windows, backslashes stays one
  word with its path intact.
- A path that starts with `-` is passed with `./` in front.
- A step whose filtered files are empty is skipped and reported as skipped, not passed. The
  entries that depend on it still run, and a run with no failed entry exits with code 0.
- An event without files filters nothing and skips nothing.
- A step whose files do not fit the command-line limit the engine is given could not run
  (exit code 2), and the reason gives the number of files and names `batch`.

## Done when

- The rules are tested in the engine core with fakes.
- The composition root passes the platform's command-line limit to the engine.
- If matching globs needs a library, the engine asks for it through a port that the
  execution context answers, as the commit-messages core asks for patterns, and its
  allow-list in `arch.toml` stays empty.
