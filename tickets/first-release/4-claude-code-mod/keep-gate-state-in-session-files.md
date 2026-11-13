---
type: ticket
title: Keep gate state in session files
blocks: tickets/first-release/4-claude-code-mod/follow-waves-and-messages-from-claude-code
parent: tickets/first-release/4-claude-code-mod/story
points: 2
priority: medium
status: pending
---

# Keep gate state in session files

## Goal

Implement `GateState` in the `continual-session-state` adapter, keeping one file per session in
the operating system's temporary directory.

## Design

The `GateState` notes in [Harness: design](../../../docs/harness.md#design).

## Behaviours to test

- The `GateState` contract suite passes against the adapter.
- Two sessions write separate files, and neither reads the other's.
- A missing or unreadable state file reads as an empty state and is reported, never as an open
  wave or an allowance.

## Done when

- `arch.toml` classifies the new crate.
