---
type: ticket
title: Check commit messages in Continual
blocks: tickets/first-release/2-built-in-checks/join-the-commit-messages-check-to-the-registry
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Check commit messages in Continual

## Goal

Have the commit-messages core check the messages CI gives it: every commit in a range,
through its `ChangeHistory` port, and a pull request's title and body.

## Design

[Built-in checks: commit messages](../../../docs/built-in-checks.md#commit-messages) and the
commit-messages row in [ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters).

## Behaviours to test

- Every commit in the range is checked, each finding naming its commit.
- A title is checked as a subject and a description as a body.
- The core's `ChangeHistory` contract suite for the messages in a range passes against the
  fake and `continual-git`.

## Done when

- The rules are tested in the commit-messages core with the fake.
