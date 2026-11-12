---
type: ticket
title: Hold the review gate in the Claude Code mod
blocks: tickets/first-release/7-self-hosting/story
order: 4
parent: tickets/first-release/epic
priority: medium
status: pending
---

# Hold the review gate in the Claude Code mod

## Goal

Ship Continual's Claude Code mod: `continual install claude` wires the hooks, and
`continual hook claude <hook>` runs the agent events, blocks commands that skip checks and
holds the review gate, all configured in `continual.toml`
([harness mods](../../../VISION.md#harness-modsplugins)).

## Design

[Harness](../../../docs/harness.md): every section.

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk).

## Done when

- Every child ticket is completed.
- `GateState` has a contract suite, a fake and the `continual-session-state` adapter.
- The design documents describe the change, and its planned markers are gone.
