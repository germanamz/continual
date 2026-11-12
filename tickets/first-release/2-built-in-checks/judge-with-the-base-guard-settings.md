---
type: ticket
title: Judge with the base revision's guard settings
blocks: tickets/first-release/2-built-in-checks/run-the-guard-on-its-own
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: medium
status: pending
---

# Judge with the base revision's guard settings

## Goal

Make the guard read `protected` and `approval-label` from the base revision's
`continual.toml`, so a change cannot loosen the guard that judges it.

## Design

The base-revision paragraph in [Built-in checks: guard](../../../docs/built-in-checks.md#guard) and
its decision in [decisions](../../../docs/built-in-checks.md#decisions).

## Behaviours to test

- A change that removes a path from `protected` is still reported for that path.
- A change that renames the approval label is still judged with the old one.
- A base revision with no `continual.toml`, or with no guard entry, makes the check
  could-not-run with that reason.

## Done when

- The configuration at a revision is read through the existing ports, with no new port.
