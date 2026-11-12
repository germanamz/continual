---
type: ticket
title: Block commands that skip checks
blocks: tickets/first-release/4-claude-code-mod/answer-claude-code-gate-hooks
parent: tickets/first-release/4-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Block commands that skip checks

## Goal

Make the core's gate block the built-in bypasses, the repository's `[[blocked_commands]]` and
any command naming the guard's approval label, each with its reason.

## Design

[Harness: blocked commands](../../../docs/harness.md#blocked-commands).

## Behaviours to test

- `--no-verify` and `-n` on a commit, a change to `core.hooksPath`, and a force push are
  blocked with their reasons.
- A command naming the guard's approval label is blocked, the label read from the guard's
  settings.
- `[[blocked_commands]]` blocks by leading `words` and by `env` assignments, with its reason.
- A blocked command stays blocked even when it is also a gated action.

## Done when

- The rules are tested in the core with segment inputs.
