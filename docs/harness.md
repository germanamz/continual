# Harness

> Planned: [Hold the review gate in the Claude Code mod](../tickets/first-release/4-claude-code-mod/story.md)

How Continual plugs into an agent harness: Claude Code, the only harness planned. The harness
mod formats
and checks each file an agent edits, runs the quick checks before an agent's turn ends, blocks
commands that skip checks, and holds the review gate: an agent commits, pushes or opens a pull
request only when a contributor asks, unless the contributor allows it
([harness mods](../VISION.md#harness-modsplugins)). Every behaviour lives in the `continual`
binary and is configured in `continual.toml`; the mod is generated wiring that calls it. Which
pipelines run is the [pipeline](pipeline.md#events)'s business. This repository's own review gate,
which these behaviours generalise, is in [Workflow](workflow.md#the-review-gate).

## Concepts

Contributor, wave and allowance mean what [Workflow](workflow.md#concepts) says. In addition:

- **Harness**: a program that runs an agent and can call hooks around its actions, such as
  Claude Code.
- **Harness mod**: the hook wiring `continual install <harness>` writes into a harness's
  settings.
- **Gated action**: a commit, a push, a pull-request creation or a merge.
- **Bypass**: a command that skips or disables checks, such as `git commit --no-verify`.
- **Gate decision**: what Continual answers about a command an agent wants to run: `allow`,
  which skips the harness's own permission prompt; `pass`, which raises no objection and
  leaves the harness to decide; `ask` the contributor; or `block`, with the reason. No answer
  lifts a deny or ask rule the contributor keeps in the harness's own settings.

## Behaviour

### Installing the mod

`continual install claude` writes Continual's hooks into `.claude/settings.json`, keeping every
entry that is not Continual's. Running it again changes nothing. Each hook runs
`continual hook claude <hook>` with the harness's payload on standard input:

| Claude Code hook | What Continual does |
| --- | --- |
| `PreToolUse` on Bash | Answers the gate decision for the command. |
| `PostToolUse` on Edit and Write | Runs the `on_agent_file_edit` event with the edited file and sends failures back to the agent. |
| `PostToolUse` and `PostToolUseFailure` on Bash | Opens a wave when an approved commit or push has run, whether it succeeded or failed. |
| `UserPromptSubmit` | Closes the wave, and reads `/allow` in the contributor's message. |
| `Stop` | Runs the `on_agent_turn_end` event and waits for it before the turn ends. |

Each hook entry that runs an event gets a timeout that covers the event's longest possible
run. Continual adds up, for every pipeline the event runs, the pipelines they depend on and
every package that has them, the timeout of each tool step and the 10 seconds a tool gets to
stop, plus an allowance for each check step, which has no timeout of its own. Claude Code
therefore never cuts a hook off before Continual's own limits stop a hung tool. A hook it did
cut off would give no answer, and the turn would end unchecked. Running
`continual install claude` again after the configuration changes brings the timeouts up to
date.

When the configuration cannot be loaded or pins another version, the gate still answers, with
the built-in rules alone: the default `ask` list and the built-in bypasses. It says so, so the
agent can still run the install command or fix the configuration. If Continual cannot answer
a `PreToolUse` hook, because it cannot read the payload or fails inside, the command is
blocked. Claude Code blocks only on a deny answer or exit code 2, so the hook turns every
failure it can catch, a panic included, into exit code 2. A hook that never starts, or that
Claude Code cuts off at its timeout, gives no answer, and Claude Code's own permission rules
then decide. The other hooks warn and let the agent continue.

### Agent events

- Continual waits for the `on_agent_turn_end` checks to finish before the agent's turn ends.
  The end of the agent's turn, when it hands back to the contributor, is the one moment that
  reliably marks finished work, so the checks run there in full even when that makes the turn
  end later: quality comes before speed.
- After an edit, the event's output goes back to the agent as the hook's feedback when a step
  fails, so the agent fixes the file. The edit itself is never undone.
- Before the turn ends, a failing `on_agent_turn_end` blocks the stop once and shows the
  agent the output. A second stop in the same turn goes through, so the agent can stop and
  report a failure it cannot fix.
- Guard findings never block a stop. The guard only warns locally
  ([Built-in checks](built-in-checks.md#guard)).

### The review gate

```toml
[review_gate]
ask = ["commit", "push", "pr", "merge"]
```

- Continual asks the contributor before every pull-request creation (`pr`) or merge, and
  before the first commit or push after each of their messages, for each gated action listed
  in `ask`. The list above is the default. An action left out of `ask` never asks.
- Approving a commit or push opens a wave, whether the command then succeeds or fails: later
  commits and pushes go through without asking until the contributor's next message. Inside a
  wave, pull requests and merges still ask and bypasses are still blocked.
- A contributor message that starts with `/allow` and names gated actions, `commit`, `push`
  or `pr`, allows them for the rest of the session, until `/allow off`. Each action is allowed
  on its own, merges are never allowed, and bypasses stay blocked. Only the contributor's own
  messages give or withdraw an allowance.
- Continual recognises a gated action whatever form the command takes: `git -C <dir>`, a full
  path to `git`, an `env` prefix, a subshell, or a command chained with `&&`, `;` or a pipe.
  Git commits and pushes are recognised by default, and so are pull requests through `gh`:
  `gh pr create`, `gh pr merge`, and a `gh api` call that writes to a repository's pulls.
  `[review_gate.commands]` adds recognisers for other tools, such as `glab mr create`.
- Continual answers `allow` only for what the contributor approved: a commit or push inside a
  wave, or a gated action their `/allow` covers, and only when the command holds nothing else
  but `cd` and `git add`. A command that chains anything else gets `pass`, and so does every
  command that is neither gated nor a bypass, so Claude Code's own permission rules decide.
- `allow` skips Claude Code's permission prompt and nothing more. Claude Code applies the deny
  and ask rules in its own settings whatever a hook answers, so an `ask` rule a contributor
  keeps there for a gated action still asks inside a wave or an allowance. A repository that
  wants waves and allowances to cover an action leaves it to `[review_gate] ask` alone, and
  `continual install claude` names every `ask` rule in the settings that matches a gated
  action.

### Blocked commands

These bypasses are always blocked: `--no-verify` on a commit or a push, `-n` on a commit, a
change to `core.hooksPath`, a force push, and any command that names the guard's approval
label. Changes to the hook files themselves, such as `.claude/settings.json`, are the guard's
business, through its `protected` list. A repository adds its own:

```toml
[[blocked_commands]]
words = ["lefthook", "uninstall"]
reason = "lefthook runs the git hooks"

[[blocked_commands]]
env = ["LEFTHOOK", "LEFTHOOK_EXCLUDE"]
reason = "these variables skip git hooks"
```

`words` matches a command that starts with those words; `env` matches a command that sets one
of those variables. A blocked command is refused with its reason, inside a wave or an
allowance too.

## Design

- **Driving adapter.** `crates/continual/src/harness/claude.rs` turns each hook payload into a
  core input and calls one core operation: the gate decision, an event run, or a
  contributor-message update. It owns Claude Code's JSON as serde DTOs (M1) and maps each
  result to Claude Code's hook response and exit code (A2). It splits a shell command into
  segments of words and variable assignments, so the core classifies words and never parses
  shell syntax.
- **Installer.** `continual install claude` lives in the same adapter. Writing hook entries
  into a settings file has no domain rule, so it adds no port (W1). It loads the workspace as
  every command does and asks the core for each event's longest run, which it writes as the
  hook's timeout.
- **Core.** The `gate` feature module classifies a command's segments as a gated action, a
  bypass or neither, from the built-in and configured recognisers, and runs the wave and
  allowance state machine. It returns a gate decision.
- **`GateState` port.** The wave and the allowances outlive one hook call, so the core reads
  and writes them through `GateState`, keyed by the harness's session ID.
  `crates/adapters/session-state` keeps one file per session in the operating system's
  temporary directory.
- **Output.** For a hook, the composition root points the `terminal` adapter at a buffer
  instead of standard output, and the harness adapter returns that buffer as the hook's
  feedback ([Pipeline](pipeline.md#ports-and-adapters)).

## Decisions

- **The behaviour is in the binary; the mod is wiring.** One implementation serves every
  harness, and a second harness is a second driving adapter and installer. Rejected: hook
  scripts per harness, like this repository's `.claude/hooks/`, which duplicate the rules and
  need `jq`.
- **Harness-neutral event names.** `on_agent_turn_end` means the same in any harness, so a
  repository's configuration does not change when it adds one.
- **The adapter splits shell commands; the core classifies them.** Shell syntax is a format,
  translated at the edge (A1). Which commands are gated or bypasses is a rule.
- **A gate that fails blocks; a gate without configuration falls back.** A hook that fails
  must not let a bypass through, so every failure it can catch becomes a block, and a broken
  or mismatched configuration must not lock the agent out of fixing it. Rejected: blocking
  every command whenever the configuration fails to load.
- **Commands are matched by their words, not by the files they write.** Recognising every
  command that could rewrite a hook file needs shell analysis Continual does not do; the
  guard reports changes to those files instead.
- **The guard's approval label is blocked automatically.** It comes from the guard's settings,
  so it is written once and cannot be forgotten in `[[blocked_commands]]`.
- **The turn-end checks run in full before the turn ends.** The agent hands back to the
  contributor only with checked work, at the cost of slower turns. Rejected: running them in
  the background, and cutting them off at a short hook timeout, either of which lets a turn end
  on unchecked work.
- **`allow` only for what the contributor approved, and never above the harness's own rules.**
  A wave and an allowance are the contributor's explicit approval, so Continual answers `allow`
  for the commands they cover and the contributor is not asked twice. Claude Code keeps its
  deny and ask rules above any hook's answer, and Continual does not work around that: a
  contributor who writes an `ask` rule there is asked. Rejected: answering `allow` to every
  command Continual has no objection to, which would skip Claude Code's prompt for commands
  nobody approved; and answering `pass` inside a wave, which leaves Claude Code asking again
  for every commit the wave was opened to cover.
- **Allowances end with the session.** They come from the contributor's messages, as in
  [Workflow](workflow.md#decisions). Rejected: allowing actions in `continual.toml` for every
  session, beyond leaving an action out of `ask`, which a repository decides for everyone.

## Open questions

- How a hook entry's timeout stays in step with the configuration between runs of
  `continual install claude`. A step timeout raised afterwards leaves the entry too short, and
  nothing reports it.
  [Install the Claude Code mod](../tickets/first-release/4-claude-code-mod/install-the-claude-code-mod.md)
  decides it.
- Whether the gate falls back to the base revision's configuration, as the guard does, instead
  of the built-in rules alone when the working tree's cannot be loaded. As it stands, an agent
  that breaks `continual.toml` drops the repository's own `[[blocked_commands]]` and
  recognisers.
  [Fall back to the built-in gate rules](../tickets/first-release/4-claude-code-mod/fall-back-to-the-built-in-gate-rules.md)
  decides it.
- Where the gate's state lives so that the agent it gates cannot write it. Any command the
  agent runs can write a file in the temporary directory, and the state holds allowances as
  well as the wave.
  [Keep gate state in session files](../tickets/first-release/4-claude-code-mod/keep-gate-state-in-session-files.md)
  decides it.

