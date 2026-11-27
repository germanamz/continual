---
type: ticket
title: Join the gate to the engine
blocks: tickets/first-release/4-claude-code-mod/answer-claude-code-gate-hooks
parent: tickets/first-release/4-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Join the gate to the engine

## Goal

Give the gate the repository's own rules: its new `GateRules` port asks for the review-gate
settings and the blocked commands, and the `continual-gate-engine` bridge answers from the
engine's workspace.

## Design

The `GateRules` note in [Harness: design](../../../docs/harness.md#design), its decision in
[decisions](../../../docs/harness.md#decisions), and the gate's sections in
[Pipeline: the engine core](../../../docs/pipeline.md#the-engine-core).

## Behaviours to test

- The engine keeps `[review_gate]` and `[[blocked_commands]]` as located values, and
  `continual-toml-config` rejects an unknown key, a wrong type or an unknown gated action in
  them with its line, for every command.
- The bridge reads them into the gate's settings.
- A configuration that cannot be loaded, or that pins another version, is answered as an
  error with the reason, and the gate's decision fails with it instead of answering.
- `GateRules` is documented per P5, and the fake and the bridge pass its contract suite.

## Done when

- The gate core is tested with the fake and knows nothing of pipelines, checks or
  `continual.toml`.
- The bridge depends on the gate core and the engine core and on no other workspace crate,
  and `arch.toml` classifies it.
