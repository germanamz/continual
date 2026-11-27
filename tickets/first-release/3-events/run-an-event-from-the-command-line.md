---
type: ticket
title: Run an event from the command line
blocks: tickets/first-release/3-events/install-git-hooks
parent: tickets/first-release/3-events/story
points: 2
priority: medium
status: pending
---

# Run an event from the command line

## Goal

Make `continual event <event> [--files <path>...] [--message-file <path>]` work end to end
through the real binary.

## Design

[Pipeline: events](../../../docs/pipeline.md#events) and
[composition root](../../../docs/pipeline.md#composition-root).

## Behaviours to test

- The real binary runs an event's mapped steps against a fixture git repository.
- `--files` gives `on_agent_file_edit` its files.
- `--message-file` gives `on_commit_msg` its message file, which reaches tool steps as
  `{message_file}` and the `commit-messages` check as the message to check.
- An unknown event exits with code 2 and lists the events the configuration maps.
- A git error while reading the event's files exits with code 2 and says why.

## Done when

- The composition root wires `continual-git` for events.
