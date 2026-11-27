---
type: ticket
title: Split subshells and variable assignments
parent: tickets/first-release/4-claude-code-mod/story
points: 2
priority: medium
status: pending
---

# Split subshells and variable assignments

## Goal

Make the Claude Code adapter's command splitting see through subshells, command substitution,
`env` prefixes and leading variable assignments, so a gated action or a bypass is recognised
whatever form the command takes.

## Design

The "whatever form the command takes" bullet in
[Harness: the review gate](../../../docs/harness.md#the-review-gate) and the adapter notes in
[Harness: design](../../../docs/harness.md#design).

## Behaviours to test

- A commit inside `( … )` or `$( … )` asks.
- `env LEFTHOOK=0 git commit` and `LEFTHOOK=0 git commit` are split into the assignment and the
  command, so `[[blocked_commands]]` with `env` blocks them.
- Quoted text that only looks like a command is not split.

## Done when

- Recorded payloads for each form run through the real binary.
