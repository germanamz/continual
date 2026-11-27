---
type: ticket
title: Block commands that skip checks
blocks:
  - tickets/first-release/5-claude-code-mod/answer-claude-code-gate-hooks
  - tickets/first-release/5-claude-code-mod/join-the-gate-to-the-engine
parent: tickets/first-release/5-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Block commands that skip checks

## Goal

Make the gate core block the built-in bypasses, the repository's blocked commands and any
command naming the approval label the gate is given, each with its reason.

## Design

[Harness: blocked commands](../../../docs/harness.md#blocked-commands).

## Behaviours to test

- `--no-verify` on a commit or a push, `-n` on a commit, a change to `core.hooksPath`, and a
  force push are blocked with their reasons. `-n` on a push, which is a dry run, is not.
- A command naming the approval label the gate is given is blocked, wherever the label
  stands in the command and whatever the tool, with no setting that turns the rule off.
- A blocked command in the gate's settings blocks by leading `words` and by `env`
  assignments, with its reason.
- A blocked command stays blocked even when it is also a gated action.

## Done when

- The rules are tested in the gate core with segment inputs and typed settings.
