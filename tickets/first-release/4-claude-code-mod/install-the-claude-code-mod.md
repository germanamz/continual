---
type: ticket
title: Install the Claude Code mod
parent: tickets/first-release/4-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Install the Claude Code mod

## Goal

Let a repository wire Continual into Claude Code with one command: `continual install claude`
writes the hooks the harness design lists into `.claude/settings.json`, with timeouts long
enough that Claude Code never cuts a hook off before Continual's own limits act.

## Design

[Harness: installing the mod](../../../docs/harness.md#installing-the-mod), the installer notes in
[Harness: design](../../../docs/harness.md#design), and the hook-timeout question in
[Harness: open questions](../../../docs/harness.md#open-questions).

## Behaviours to test

- Into a settings file with other hooks and settings, Continual adds its own hooks and changes
  nothing else.
- Running it again changes nothing.
- Without a settings file, it creates one.
- Each hook entry calls `continual hook claude <hook>` for the hooks the design's table lists.

## Done when

- The hook-timeout open question in docs/harness.md is decided and removed.
- The design documents describe the change, and the planned markers of
  [Hold the review gate in the Claude Code mod](story.md)
  are gone.
