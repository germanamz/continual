---
type: ticket
title: Check commit messages in Continual
blocks: tickets/first-release/3-events/give-check-steps-an-events-files-and-message
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Check commit messages in Continual

## Goal

Run the commit-messages check on the messages CI gives it: every commit in `--range`, and a
pull request's `--title` and `--body`.

## Design

[Built-in checks: commit messages](../../../docs/built-in-checks.md#commit-messages) and the
`ChangeHistory` row in [ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- Every commit in `--range` is checked, each finding naming its commit.
- `--title` is checked as a subject and `--body` as a body.
- The `ChangeHistory` contract suite for the messages in a range passes against the fake and
  `continual-git`.
- The real binary runs a `commit-messages` check step on a fixture repository.

## Done when

- The design documents describe the change, and the planned markers of
  [Run the built-in checks in Continual](story.md) are gone
  once its other children are completed.
