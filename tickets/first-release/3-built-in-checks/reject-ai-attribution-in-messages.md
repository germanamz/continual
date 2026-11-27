---
type: ticket
title: Reject AI attribution in messages
blocks: tickets/first-release/3-built-in-checks/check-message-patterns
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Reject AI attribution in messages

## Goal

Bring the `xtask` attribution rule into the commit-messages core, on by default and turned off
by its `attribution` setting.

## Design

The attribution bullet in [Built-in checks: commit messages](../../../docs/built-in-checks.md#commit-messages).

## Carried over

- These are the first findings of the commit-messages core. A `Finding` is a file, a line and
  a message, and a message in a commit range or a pull request's title is in no file. Where
  such a finding is, is open
  ([Built-in checks: open questions](../../../docs/built-in-checks.md#open-questions)), and
  is settled by the task that declares `Finding`. If it is still open here, settle it with
  the contributor first.

## Behaviours to test

- Each case the `xtask` attribution tests cover gives the same findings: AI co-author
  trailers, session trailers and links, and "generated with" lines are rejected; human
  co-authors and ordinary prose pass.
- With attribution turned off, no attribution findings.

## Done when

- Existing: the `xtask` attribution check keeps passing its tests unchanged.
- The rule is tested in the commit-messages core through its public API.
