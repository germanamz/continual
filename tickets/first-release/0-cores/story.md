---
type: ticket
title: Hold Continual's rules in several cores
blocks:
  - tickets/first-release/1-pipeline/story
  - tickets/first-release/2-built-in-checks/story
  - tickets/first-release/4-claude-code-mod/story
order: 0
parent: tickets/first-release/epic
priority: high
status: pending
---

# Hold Continual's rules in several cores

## Goal

Let the workspace hold several cores, each one unit of Continual's rules in a crate of its
own that depends on no other, so every rule stays short enough to read and test whole and the
boundary between two units is one that the compiler and `cargo xtask arch` hold
([cores](../../../CONVENTIONS.md#cores-h)).

## Design

- [Workspace: planned crates](../../../docs/workspace.md#planned-crates) and the decisions on
  cores in [decisions](../../../docs/workspace.md#decisions).
- The planned `arch` paragraph in
  [Checks: `cargo xtask` checks](../../../docs/checks.md#cargo-xtask-checks).
- [Continual's cores](../../../CONVENTIONS.md#3-continuals-cores) and rules H1 to H5.

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk).

## Done when

- Every child ticket is completed.
- A second core can be added under `crates/cores/`, and `cargo xtask arch` holds its boundary.
- The design documents and `CONVENTIONS.md` describe the workspace as it is, and the planned
  markers of this story are gone.
