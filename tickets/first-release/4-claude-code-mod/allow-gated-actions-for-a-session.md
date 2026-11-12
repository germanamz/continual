---
type: ticket
title: Allow gated actions for a session
blocks: tickets/first-release/4-claude-code-mod/follow-waves-and-messages-from-claude-code
parent: tickets/first-release/4-claude-code-mod/story
points: 2
priority: medium
status: pending
---

# Allow gated actions for a session

## Goal

Add `/allow` allowances to the core's gate: a contributor message can allow commits, pushes or
pull requests for the rest of the session, until `/allow off`.

## Design

The `/allow` bullet in [Harness: the review gate](../../../docs/harness.md#the-review-gate).

## Behaviours to test

- `/allow commit`, `/allow push` and `/allow pr` each allow one action, alone or together.
- `/allow off` withdraws every allowance.
- Merges always ask, and bypasses stay blocked, under any allowance.
- Only a contributor message gives or withdraws an allowance.
- An allowance never reaches another session.

## Done when

- The rules are tested in the core with the `GateState` fake.
