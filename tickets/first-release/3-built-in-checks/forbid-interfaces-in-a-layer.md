---
type: ticket
title: Forbid interfaces in a layer
blocks:
  - tickets/first-release/3-built-in-checks/read-rust-traits
  - tickets/first-release/3-built-in-checks/load-units-from-the-source-tree
parent: tickets/first-release/3-built-in-checks/story
points: 2
priority: medium
status: pending
---

# Forbid interfaces in a layer

## Goal

Add the `interfaces` rule to the architecture core, over the interfaces a language adapter
reports, with the interfaces operation added to its `LanguageSyntax`, so a repository can
declare a layer that holds data and no ports, as Continual's library crates are.

## Design

The `interfaces` paragraph in
[Built-in checks: architecture](../../../docs/built-in-checks.md#architecture) and the
decision "A layer can forbid interfaces" in
[decisions](../../../docs/built-in-checks.md#decisions).

## Carried over

- An agent chose the key's name and shape, `interfaces = false`, from the contributor's
  answer that the check needs a new key for library crates
  ([MEMORY.md](../../../MEMORY.md)). Confirm it before building the rule.

## Behaviours to test

- In a layer with `interfaces = false`, an interface declared in one of the layer's source
  files is reported at its line.
- A layer that leaves `interfaces` out, or sets it to `true`, may declare interfaces.
- The fake passes the `LanguageSyntax` contract suite for interfaces.

## Done when

- The rule is tested in the architecture core with the fake.
- This task's planned marker is gone from `docs/built-in-checks.md`.
