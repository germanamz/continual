---
type: ticket
title: Open and close review waves
blocks:
  - tickets/first-release/4-claude-code-mod/keep-gate-state-in-session-files
  - tickets/first-release/4-claude-code-mod/allow-gated-actions-for-a-session
parent: tickets/first-release/4-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Open and close review waves

## Goal

Add the review gate's waves to the core, kept between hook calls through the new `GateState`
port: an approved commit or push opens a wave, and the contributor's next message closes it.

## Design

[Harness: the review gate](../../../docs/harness.md#the-review-gate) and the `GateState` notes in
[Harness: design](../../../docs/harness.md#design).

## Behaviours to test

- The first commit or push after a contributor message asks; once one is approved, later
  commits and pushes are allowed until the contributor's next message.
- Inside a wave, pull requests and merges still ask, and bypasses stay blocked.
- State is kept per session ID: one session's wave never reaches another.
- `GateState` is documented per P5, and the fake passes its contract suite.

## Done when

- The rules are tested in the core with the fake.
