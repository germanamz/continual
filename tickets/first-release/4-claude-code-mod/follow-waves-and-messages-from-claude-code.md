---
type: ticket
title: Follow waves and messages from Claude Code
blocks: tickets/first-release/4-claude-code-mod/install-the-claude-code-mod
parent: tickets/first-release/4-claude-code-mod/story
points: 2
priority: medium
status: pending
---

# Follow waves and messages from Claude Code

## Goal

Add `continual hook claude post-tool-use`, for Bash commands that succeed or fail, and
`user-prompt-submit`, so an approved commit or push opens a wave, and each contributor message
closes it and is read for `/allow`.

## Design

The hook table in [Harness: installing the mod](../../../docs/harness.md#installing-the-mod) and
[the review gate](../../../docs/harness.md#the-review-gate).

## Behaviours to test

- `PostToolUse` or `PostToolUseFailure` after an approved commit or push opens a wave for the
  session, so a commit that fails its checks does not ask again when it is retried.
- `UserPromptSubmit` closes the wave, and gives or withdraws allowances from `/allow`.
- Text that is not the contributor's own message never gives an allowance.

## Done when

- The real binary runs both hooks with recorded payloads, with `continual-session-state`
  wired in by the composition root.
