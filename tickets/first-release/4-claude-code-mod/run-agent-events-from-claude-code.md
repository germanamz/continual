---
type: ticket
title: Run agent events from Claude Code
blocks: tickets/first-release/4-claude-code-mod/install-the-claude-code-mod
parent: tickets/first-release/4-claude-code-mod/story
points: 3
priority: medium
status: pending
---

# Run agent events from Claude Code

## Goal

Run the agent events from Claude Code's hooks: `post-tool-use` on Edit and Write runs
`on_agent_file_edit` with the edited files, and `stop` runs `on_agent_turn_end`, with the run's
output sent back to the agent as the hook's feedback.

## Design

[Harness: agent events](../../../docs/harness.md#agent-events) and the output notes in
[Harness: design](../../../docs/harness.md#design).

## Behaviours to test

- An edit whose mapped steps fail returns their output to the agent as feedback; one whose
  steps pass returns nothing; the edit is never undone.
- The `stop` hook answers only once every pipeline of `on_agent_turn_end` has finished, so the
  turn never ends on checks still running.
- A failing `on_agent_turn_end` blocks the first stop with its output, and a second stop in the
  same turn goes through.
- Guard findings never block a stop.
- The output the hook returns is the same text the terminal would show.

## Done when

- The real binary runs both hooks with recorded payloads against a fixture repository.
