---
type: ticket
title: Read Rust comments
blocks: tickets/first-release/2-built-in-checks/run-check-steps
parent: tickets/first-release/2-built-in-checks/story
points: 3
priority: high
status: pending
---

# Read Rust comments

## Goal

Create the `continual-rust` adapter and implement the comments operation of `LanguageSyntax`
for Rust with `ra-ap-rustc_lexer`, so the core's comments rule works on Rust as the `xtask`
check does today.

## Design

The `rust` notes in [Built-in checks: ports and adapters](../../../docs/built-in-checks.md#ports-and-adapters)
and [comments](../../../docs/built-in-checks.md#comments).

## Behaviours to test

- The `LanguageSyntax` contract suite for comments passes against the adapter.
- Each case the `xtask` comments tests cover gives the same findings through the core rule and
  this adapter: comment markers inside strings, raw strings and byte strings are never
  comments; a comment ending in `;`, `{` or `}` is code; CRLF files read like LF files.
- The adapter states the `.rs` extension.

## Done when

- `arch.toml` classifies the new crate, and it uses `ra-ap-rustc_lexer` from the workspace
  dependencies.
- Existing: the `xtask` comments check keeps passing its tests unchanged.
