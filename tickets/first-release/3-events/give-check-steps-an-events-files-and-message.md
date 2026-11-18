---
type: ticket
title: Give check steps an event's files and message
blocks: tickets/first-release/3-events/run-an-event-from-the-command-line
parent: tickets/first-release/3-events/story
points: 2
priority: medium
status: pending
---

# Give check steps an event's files and message

## Goal

Let check steps take what an event carries: per-file checks inspect only the event's files, and
the `commit-messages` check reads the `on_commit_msg` message file, so an event checks exactly
what the moment touched.

## Design

The per-file and repository checks in
[Built-in checks: concepts](../../../docs/built-in-checks.md#concepts), the no-vacuous-pass
bullet in [check steps](../../../docs/built-in-checks.md#check-steps),
[commit messages](../../../docs/built-in-checks.md#commit-messages) and
[Pipeline: events](../../../docs/pipeline.md#events).

## Behaviours to test

- In an event with files, a per-file check inspects only the files of its package among them.
- With none of them in a supported language, the per-file check is skipped, not passed, and
  the repository checks of its step still run.
- A check step is skipped only when every check in it was, and a skipped step does not fail
  the run.
- In an event without files, a per-file check inspects every file of its package.
- A repository check ignores an event's files.
- On `on_commit_msg`, the `commit-messages` check checks the event's message file.

## Done when

- The rules are tested in the core with fakes.
