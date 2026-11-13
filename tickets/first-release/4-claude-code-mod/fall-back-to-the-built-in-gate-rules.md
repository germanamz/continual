---
type: ticket
title: Fall back to the built-in gate rules
parent: tickets/first-release/4-claude-code-mod/story
points: 1
priority: medium
status: pending
---

# Fall back to the built-in gate rules

## Goal

When the configuration cannot be loaded or pins another version, answer `PreToolUse` with the
built-in gate rules instead of blocking, so the agent can still fix the configuration or
install the pinned release.

## Design

The fallback paragraph in [Harness: installing the mod](../../../docs/harness.md#installing-the-mod).

## Behaviours to test

- With an invalid configuration, the default `ask` list and the built-in bypasses apply, and
  the answer says the configuration could not be loaded.
- With a pinned version that is not the binary's, the same rules apply and the answer names
  both versions.

## Done when

- Recorded payloads run through the real binary against fixture configurations.
