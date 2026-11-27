---
type: ticket
title: Run the built-in checks in Continual
blocks:
  - tickets/first-release/7-self-hosting/story
  - tickets/first-release/6-releases/publish-the-first-release
order: 2
parent: tickets/first-release/epic
priority: high
status: pending
---

# Run the built-in checks in Continual

## Goal

Bring the comments, architecture, guard and commit-message checks from `xtask` into
Continual as built-in checks that check steps run, each rule in a core of its own behind the
check registry, and Rust supported through the first language adapter ([rules](../../../VISION.md#rules),
[languages](../../../VISION.md#languages)).

## Design

[Built-in checks](../../../docs/built-in-checks.md): every section.

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk). Throughout, the `xtask` checks keep passing their
tests unchanged.

## Done when

- Every child ticket is completed.
- Every port the registry and the check cores declare has a contract suite, a fake and its
  adapter or bridge.
- The design documents describe the change, and its planned markers are gone.
