---
type: ticket
title: Run the guard on its own
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Run the guard on its own

## Goal

Add `continual guard --ci --labels <list>`: the check registry runs one check by name outside
any step, with the base revision's settings, so CI runs the guard even when a change removes
it from the steps.

## Design

The `continual guard` paragraph in [Built-in checks: guard](../../../docs/built-in-checks.md#guard)
and [the registry core](../../../docs/built-in-checks.md#the-registry-core).

## Behaviours to test

- The real binary runs the guard with no step naming it and reports its findings.
- With `--ci`, it fails unless `--labels` holds the approval label.
- With no guard entry at the base revision, it runs with the built-in settings.
- With several guard entries at the base revision, it runs each with its own settings.

## Done when

- The command calls one operation of the registry core (A2), which is tested there with a
  fake check.
