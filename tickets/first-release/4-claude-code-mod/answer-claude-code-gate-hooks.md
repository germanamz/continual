---
type: ticket
title: Answer Claude Code before each command
blocks:
  - tickets/first-release/4-claude-code-mod/run-agent-events-from-claude-code
  - tickets/first-release/4-claude-code-mod/split-subshells-and-variable-assignments
  - tickets/first-release/4-claude-code-mod/fall-back-to-the-built-in-gate-rules
  - tickets/first-release/4-claude-code-mod/follow-waves-and-messages-from-claude-code
parent: tickets/first-release/4-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Answer Claude Code before each command

## Goal

Add the Claude Code driving adapter's first hook: `continual hook claude pre-tool-use` reads
Claude Code's payload, splits the Bash command into segments, asks the core's gate, and answers
allow, ask or block in Claude Code's hook format.

## Design

[Harness: installing the mod](../../../docs/harness.md#installing-the-mod), the failure rules there,
and [Harness: design](../../../docs/harness.md#design).

## Behaviours to test

- Recorded `PreToolUse` payloads for a commit, a push, a pull request, a bypass and an ordinary
  command get the matching allow, ask or block response.
- A command chained with `&&`, `;` or a pipe is split into its segments.
- A payload with fields this version does not know is still answered.
- A payload Continual cannot read blocks the command.

## Done when

- The adapter is tested by running the real binary with recorded payloads (T7).
- The shell-splitting choice matches the one the `toml-config` adapter made.
