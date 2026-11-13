---
type: ticket
title: Reject AI attribution in messages
blocks: tickets/first-release/2-built-in-checks/check-message-patterns
parent: tickets/first-release/2-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Reject AI attribution in messages

## Goal

Bring the `xtask` attribution rule into the core as the commit-messages check's attribution
rule, on by default and turned off with `attribution = false`.

## Design

The attribution bullet in [Built-in checks: commit messages](../../../docs/built-in-checks.md#commit-messages).

## Behaviours to test

- Each case the `xtask` attribution tests cover gives the same findings: AI co-author
  trailers, session trailers and links, and "generated with" lines are rejected; human
  co-authors and ordinary prose pass.
- With `attribution = false`, no attribution findings.

## Done when

- Existing: the `xtask` attribution check keeps passing its tests unchanged.
