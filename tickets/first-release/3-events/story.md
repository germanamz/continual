---
type: ticket
title: Run steps on git and agent events
blocks:
  - tickets/first-release/4-claude-code-mod/story
  - tickets/first-release/7-self-hosting/story
order: 3
parent: tickets/first-release/epic
priority: medium
status: pending
---

# Run steps on git and agent events

## Goal

Let a repository map moments such as a commit, an agent's file edit or the end of an agent's
turn to the steps they run, with the touched files passed to the tools, so git hooks and
agent harnesses run the same steps through one implementation
([hooks](../../../VISION.md#hooks)).

## Design

[Pipeline: placeholders](../../../docs/pipeline.md#placeholders),
[events](../../../docs/pipeline.md#events) and the `events` module in
[Pipeline: core](../../../docs/pipeline.md#core).

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk).

## Done when

- Every child ticket is completed.
- A git hook and a scripted agent event each run their mapped steps on the right files.
- The design documents describe the change, and its planned markers are gone.
