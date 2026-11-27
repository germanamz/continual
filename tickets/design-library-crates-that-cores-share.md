---
type: ticket
title: Design library crates that cores share
points: 2
priority: high
status: pending
---

# Design library crates that cores share

## Goal

Decide how cores reuse data structures instead of each declaring its own copy. Today cores
share no crate (H4): the engine's `Location` and `Problem`, and the name rule that `CheckName`
and `StepName` both follow, are written in the core that uses them, each core to come adds
findings and located values of its own, and every bridge maps one copy to the other. The
direction to explore is a third kind of crate, a library crate that is neither a core nor an
adapter, which holds data structures that cores, adapters and composition roots all use.
This is a spike: it ends with the design and the tasks that build it, and builds no crate
itself.

## Design

- The open question in [Workspace: open questions](../docs/workspace.md#open-questions), and
  the decisions "Several small cores, joined by bridges" and "Cores share no crate" in
  [Workspace: decisions](../docs/workspace.md#decisions), which this spike reopens.
- Rules B1, B7, H1 and H4 in [CONVENTIONS.md](../CONVENTIONS.md#cores-h).
- The `arch` paragraph in
  [Checks: `cargo xtask` checks](../docs/checks.md#cargo-xtask-checks), which today reports a
  core that depends on any workspace crate.

## Questions to answer

- What a library crate may hold: plain data records only, or value objects with their rules
  too, such as the name rule. What keeps one core's own rule out of it (Q16).
- What a library crate may depend on, and what may depend on it. Whether it keeps a core's
  limits: no technology dependency, no I/O, an allow-list and a `clippy.toml` of its own.
- How `arch.toml` classifies a library crate and what `cargo xtask arch` checks for it, so the
  new role is not a way round the boundaries between cores.
- Which types move first and which stay in their core. The engine's `ProblemKind` names the
  engine's own rules, so by nature only a location and the `path:line: message` shape are
  shared.
- What a bridge still maps once two cores speak a shared type.
- How the cost the present decision names is bounded: a crate every core depends on and none
  can change alone.
- What one more published crate costs a release ([Distribution](../docs/distribution.md)).

## Done when

- The contributor has settled the question from an example, the ways to settle it with their
  costs, and a suggestion.
- The design documents hold the answer: the decision in docs/workspace.md with the
  alternatives rejected, the planned `arch` rules in docs/checks.md, and the open question
  gone.
- The changes to `CONVENTIONS.md`, `AGENTS.md` and the agent rules are in the same pull
  request, for the contributor to approve as guarded changes.
- The work to build it is in tickets: the new role in the arch check, the first library
  crate, and each move of a type out of a core.
