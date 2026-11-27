---
type: ticket
title: Split a command line into words
blocks: tickets/first-release/2-pipeline/read-continual-toml
parent: tickets/first-release/2-pipeline/story
points: 2
priority: high
status: pending
---

# Split a command line into words

## Goal

Create `continual-shell-words`, the one tokenizer that splits a command line the way a shell
does: words, quoted text kept together, and shell operators as tokens of their own. The
configuration adapter and the harness adapter both build on it, so quotes mean the same in a
step's `run` and in the command an agent wants to run.

## Design

[Pipeline: commands](../../../docs/pipeline.md#commands), the crate in
[Workspace: planned crates](../../../docs/workspace.md#planned-crates) and the tokenizer
decision in [decisions](../../../docs/workspace.md#decisions).

## Behaviours to test

- Words are split on spaces and tabs, and single quotes, double quotes and a backslash keep
  text together, as a POSIX shell reads them.
- An unclosed quote is an error that says where the quote opened.
- `&&`, `||`, `;`, `|`, `&`, `<`, `>` and parentheses outside quotes come back as operator
  tokens, with or without spaces around them. Inside quotes they are part of a word.
- Each token carries its position in the line, so a caller can point at it.

## Done when

- `arch.toml` classifies the new crate under `crates/adapters/`, which depends on no other
  workspace crate and on no new external one.
- The tokenizer is tested through its public API. [Read continual.toml](read-continual-toml.md)
  is its first caller.
