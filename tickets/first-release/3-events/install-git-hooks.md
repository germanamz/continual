---
type: ticket
title: Install git hooks that run events
parent: tickets/first-release/3-events/story
points: 2
priority: medium
status: pending
---

# Install git hooks that run events

## Goal

Let a repository without a hook manager run its git events: `continual install git` writes
`pre-commit`, `commit-msg` and `pre-push` hooks that call `continual event`.

## Design

[Pipeline: events](../../../docs/pipeline.md#events).

## Behaviours to test

- `continual install git` writes a hook for each git event the configuration maps, into the
  directory git uses for hooks, `core.hooksPath` included.
- Each hook calls `continual event` with the event's name, and `commit-msg` passes the message
  file.
- Running it again changes nothing.
- A hook file that Continual did not write is left alone, and the command says which.

## Done when

- The real binary installs hooks in a fixture repository, and a commit there runs the mapped
  steps.
- The design documents describe the change, and the planned markers of
  [Run steps on git and agent events](story.md) are gone.
