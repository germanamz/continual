---
type: ticket
title: Share data between cores in library crates
order: 1
parent: tickets/first-release/epic
priority: high
status: pending
---

# Share data between cores in library crates

## Goal

Give the workspace its third kind of product crate, the library crate, so that data several
cores mean the same way has one definition instead of a copy in each core and a mapping in
each bridge, without opening a way round the boundaries between cores
([cores](../../../CONVENTIONS.md#cores-h)).

## Design

- [Workspace: library crates](../../../docs/workspace.md#library-crates) and the decisions on
  them in [decisions](../../../docs/workspace.md#decisions).
- [Checks: library crates](../../../docs/checks.md#library-crates) and its decisions in
  [decisions](../../../docs/checks.md#decisions).
- Rules H4 and H6 to H8 in [CONVENTIONS.md](../../../CONVENTIONS.md#cores-h).

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk).

## Done when

- Every child ticket is completed.
- `continual-vocabulary` holds `Location`, `Located<T>` and the name rule, and no core
  declares a copy of them.
- `cargo xtask arch`, `cargo xtask guard` and `cargo xtask coverage` hold a library crate to
  rules H6 to H8.
- The design documents and `CONVENTIONS.md` describe the workspace as it is, and the planned
  markers of this story are gone.
