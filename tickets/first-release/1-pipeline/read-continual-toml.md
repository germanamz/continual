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
`continual.toml`, read it into the engine's configuration input, and report every malformed key
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
- `run` is split into words with `continual-shell-words`, with single and double quotes
  grouping words, and `{files}` and `{message_file}` kept as placeholder tokens. An unclosed
  quote, and a shell operator outside quotes such as `&&`, `|` or `>`, are configuration
  errors at the step's line that name it.
- `[steps]` and `[pipelines]` are read, each pipeline's `[[pipelines.<name>.steps]]` entries
  in the order written, with their `step`, `depends_on` and `required`.
- `[continual] version` is read even when the rest of the file holds keys this binary does not
  know, so the version check can come first.
- CRLF line endings read the same as LF.

## Done when

- `arch.toml` classifies the new crate and declares its coupling to `continual-shell-words`
  with the reason.
