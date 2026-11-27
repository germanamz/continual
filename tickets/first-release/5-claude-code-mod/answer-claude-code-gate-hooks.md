---
type: ticket
title: Answer Claude Code before each command
blocks:
  - tickets/first-release/5-claude-code-mod/run-agent-events-from-claude-code
  - tickets/first-release/5-claude-code-mod/split-subshells-and-variable-assignments
parent: tickets/first-release/5-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Answer Claude Code before each command

## Goal

Add the Claude Code driving adapter's first hook: `continual hook claude pre-tool-use` reads
Claude Code's payload, splits the Bash command into segments, asks the gate core, and answers
in Claude Code's hook format.

## Design

[Harness: installing the mod](../../../docs/harness.md#installing-the-mod), the failure rules there,
and [Harness: design](../../../docs/harness.md#design).

## Behaviours to test

- Recorded `PreToolUse` payloads for a commit, a push, a pull request, a bypass and an ordinary
  command get the matching response: `ask` as an ask decision, `block` as a deny with its
  reason, and `pass` as no decision, so Claude Code's own permission rules still apply.
- The same payload gets the same answer however many commands came before it: the hook keeps
  nothing between calls.
- A command chained with `&&`, `;` or a pipe is split into its segments.
- A payload with fields this version does not know is still answered.
- A payload Continual cannot read, and a failure inside the hook, a panic included, each block
  the command by ending with exit code 2.
- With an invalid configuration, every command is blocked and the reason gives the
  configuration's problems. With a pinned version that is not the binary's, every command is
  blocked and the reason names both versions and the install command.

## Done when

- The adapter is tested by running the real binary with recorded payloads (T7).
- The command is split with `continual-shell-words`, the tokenizer `continual-toml-config`
  uses.
- The composition root gives the gate the approval label it reads from the guard core.
