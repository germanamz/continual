---
type: ticket
title: Run the built-in checks in Continual
blocks: tickets/first-release/7-self-hosting/story
order: 2
parent: tickets/first-release/epic
priority: high
status: pending
---

# Run the built-in checks in Continual

## Goal

Bring the comments, architecture, guard and commit-message checks from `xtask` into
Continual as built-in checks that check steps run, with one implementation of each rule in the
core and Rust supported through the first language adapter ([rules](../../../VISION.md#rules),
[languages](../../../VISION.md#languages)).

## Design

[Built-in checks](../../../docs/built-in-checks.md): every section.

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk). Throughout, the `xtask` checks keep passing their
tests unchanged.

## Done when

- Every child ticket is completed.
- `SourceTree`, `ChangeHistory` and `LanguageSyntax` have contract suites, fakes and adapters.
- The design documents describe the change, and its planned markers are gone.
