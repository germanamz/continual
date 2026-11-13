---
type: ticket
title: Run the guard on its own
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Run the guard on its own

## Goal

Add `continual guard --ci --labels <list>`, which runs the guard outside any step with the
base revision's settings, so CI runs it even when a change removes it from the steps.

## Design

The `continual guard` sentence in [Built-in checks: guard](../../../docs/built-in-checks.md#guard).

## Behaviours to test

- The real binary runs the guard with no step naming it and reports its findings.
- With `--ci`, it fails unless `--labels` holds the approval label.
- It fails when the base revision's configuration has no guard entry.

## Done when

- The command is a driving-adapter change only, calling one core operation (A2).
