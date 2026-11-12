---
type: ticket
title: Read continual.toml
blocks: tickets/first-release/1-pipeline/run-pipelines-from-the-command-line
parent: tickets/first-release/1-pipeline/story
points: 3
priority: high
status: pending
---

# Read continual.toml

## Goal

Implement `ConfigSource` in the `continual-toml-config` adapter: find the root
`continual.toml`, read it into the core's configuration input, and report every malformed key
or value at its line.

## Design

[Pipeline: `continual.toml`](../../../docs/pipeline.md#continualtoml),
[commands](../../../docs/pipeline.md#commands), and the `toml-config` notes in
[ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- The `ConfigSource` contract suite passes against the adapter.
- From any directory below the root, the adapter finds the root configuration; with none
  above, it says no `continual.toml` was found.
- An unknown key, a value of the wrong type, and an unparseable duration are each reported as
  `continual.toml:line: message`.
- `run` is split into words like a shell would split them, with single and double quotes
  grouping words, and `{files}` and `{message_file}` kept as placeholder tokens; an unclosed
  quote is a configuration error.
- `[steps]` and `[pipelines]` are read with their inline entry tables, each entry's
  `depends_on` and `required` included.
- CRLF line endings read the same as LF.

## Done when

- `arch.toml` classifies the new crate.
- The shell-word splitting choice in the open questions of docs/workspace.md is decided, with
  any new dependency added per G8, and the question removed.
