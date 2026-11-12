---
type: ticket
title: Read an event's files from the change history
blocks: tickets/first-release/3-events/read-staged-and-uncommitted-files-in-git
parent: tickets/first-release/3-events/story
points: 2
priority: medium
status: pending
---

# Read an event's files from the change history

## Goal

Have the event use case read the files of `on_pre_commit` and `on_agent_turn_end` through
`ChangeHistory`: the staged files, and every file with uncommitted changes.

## Design

[Pipeline: events](../../../docs/pipeline.md#events) and the `ChangeHistory` row in
[Pipeline: ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- `on_pre_commit` runs its steps with the staged files.
- `on_agent_turn_end` runs its steps with every file that has uncommitted changes, untracked
  files included, and with none it runs nothing and passes.
- `on_agent_file_edit` takes the files it is given and never asks the change history.
- The `ChangeHistory` fake passes the contract suite for the staged and uncommitted files.

## Done when

- The operations are added to `ChangeHistory`, documented per P5; the port is created here if
  it does not exist yet.
