# Harness

> Planned: [Hold the review gate in the Claude Code mod](../tickets/first-release/5-claude-code-mod/story.md)

How Continual plugs into an agent harness: Claude Code, the only harness planned. The harness
mod formats
and checks each file an agent edits, runs the quick checks before an agent's turn ends, blocks
commands that skip checks, and holds the review gate: an agent commits, pushes or opens a pull
request only when a contributor approves it, unless the repository's configuration leaves that
action out of the gate
([harness mods](../VISION.md#harness-modsplugins)). Every behaviour lives in the `continual`
binary and is configured in `continual.toml`; the mod is generated wiring that calls it. Which
pipelines run is the [pipeline](pipeline.md#events)'s business. This repository's own review gate,
the shell hooks that the mod replaces, is in [Workflow](workflow.md#the-review-gate).

## Concepts

Contributor means what [Workflow](workflow.md#concepts) says. In addition:

- **Harness**: a program that runs an agent and can call hooks around its actions, such as
  Claude Code.
- **Harness mod**: the hook wiring `continual install <harness>` writes into a harness's
  settings.
- **Gated action**: a commit, a push, a pull-request creation or a merge.
- **Bypass**: a command that skips or disables checks, such as `git commit --no-verify`.
- **Gate decision**: what Continual answers about a command an agent wants to run: `pass`,
  which raises no objection and leaves the harness to decide; `ask` the contributor; or
  `block`, with the reason. No answer lifts a deny or ask rule the contributor keeps in the
  harness's own settings.

## Behaviour

### Installing the mod

`continual install claude` writes Continual's hooks into `.claude/settings.json`, keeping every
entry that is not Continual's. Running it again changes nothing. Each hook runs
`continual hook claude <hook>` with the harness's payload on standard input:

| Claude Code hook | What Continual does |
| --- | --- |
| `PreToolUse` on Bash | Answers the gate decision for the command. |
| `PostToolUse` on Edit and Write | Runs the `on_agent_file_edit` event with the edited file and sends failures back to the agent. |
| `Stop` | Runs the `on_agent_turn_end` event and waits for it before the turn ends. |

Each hook entry that runs an event gets a timeout that covers the event's longest possible
run. Continual adds up, for every pipeline the event runs, the pipelines they depend on and
every package that has them, the timeout of each step and the 10 seconds a tool gets to
stop. Claude Code therefore never cuts a hook off before Continual's own limits stop a hung
step. A hook it did cut off would give no answer, and the turn would end unchecked. That
timeout is only as current as the last run of the installer
([keeping the mod up to date](#keeping-the-mod-up-to-date)).

The gate answers only from a configuration it could load. When `continual.toml` cannot be
loaded or pins another version, the command is blocked, and the reason gives the
configuration's problems, or the two versions and the install command. The agent can still
repair the file, since its edit tools pass through no gate hook, and the contributor installs
the pinned release. The same holds whenever Continual cannot answer a `PreToolUse` hook,
because it cannot read the payload or fails inside: the command is blocked. Claude Code blocks
only on a deny answer or exit code 2, so the hook turns every failure it can catch, a panic
included, into exit code 2. A hook that never starts, or that Claude Code cuts off at its
timeout, gives no answer, and Claude Code's own permission rules then decide. The other hooks
warn and let the agent continue.

### Keeping the mod up to date

**After changing the configuration, run `continual install claude` again.** The mod does not
follow `continual.toml` on its own.

The timeout in a hook entry is a copy. Claude Code applies it, and Claude Code reads only
`.claude/settings.json`: it knows nothing of `continual.toml`, and a running hook can neither
see nor change its own timeout. Continual reads `continual.toml` on every run, so a changed
step or pipeline takes effect at once. The hook's timeout does not: it stays as the installer
last wrote it.

Run the installer again after a change to any of these:

- a step's `timeout`, or `[continual] timeout`;
- the pipelines an agent event runs, in `[events]`;
- the entries of those pipelines, or the pipelines they depend on;
- the repository's packages, or a package's own steps and pipelines.

Take a repository whose `quick` pipeline holds one step, `fmt`, with the default limit of 30
minutes, so the installer wrote the `Stop` hook with a timeout of 30 minutes and 10 seconds.
A later change adds `test`, with `timeout = "40m"`, to `quick`, and nobody runs the installer.
Continual would stop a hung `test` at 40 minutes and report the failure. Claude Code gives up
on the hook at 30 minutes and 10 seconds instead, the hook gives no answer, and the agent's
turn ends without the checks' verdict. Nothing reports it: Continual cannot tell that it was
cut off.

Running the installer again is always safe, since it changes only Continual's own entries.
Commit the updated `.claude/settings.json` together with the change to `continual.toml`, so
every contributor's agent gets both.

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

- Continual asks the contributor before every gated action listed in `ask`: each commit, each
  push, each pull-request creation (`pr`) and each merge. The list above is the default. An
  action left out of `ask` never asks, which is how a repository lets its agents take that
  action without a question.
- The gate keeps nothing between one command and the next. Each command is answered from its
  own words and the configuration, so approving one commit says nothing about the next one.
  What a harness remembers about an approval is the harness's business.
- Continual recognises a gated action whatever form the command takes: `git -C <dir>`, a full
  path to `git`, an `env` prefix, a subshell, or a command chained with `&&`, `;` or a pipe.
  Git commits and pushes are recognised by default, and so are pull requests through `gh`:
  `gh pr create`, `gh pr merge`, and a `gh api` call that writes to a repository's pulls.
  `[review_gate.commands]` adds recognisers for other tools, such as `glab mr create`.
- A command that is neither a gated action nor a bypass gets `pass`, and so does a gated
  action left out of `ask`, so Claude Code's own permission rules decide. Claude Code applies
  the deny and ask rules in its own settings whatever a hook answers.

### Blocked commands

These bypasses are always blocked: `--no-verify` on a commit or a push, `-n` on a commit, a
change to `core.hooksPath`, a force push, and any command that names the guard's approval
label, `guard-approved`. Changes to the hook files themselves, such as `.claude/settings.json`, are the guard's
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
of those variables. A blocked command is refused with its reason, whether or not it is also a
gated action.

## Design

- **Driving adapter.** `crates/continual/src/harness/claude.rs` turns each hook payload into a
  core input and calls one operation of one core: the gate's decision or the engine's event
  run. It owns Claude Code's JSON as serde DTOs (M1) and maps each result to Claude Code's
  hook response and exit code (A2). It splits a shell command into segments of words and
  variable assignments with the `shell-words` tokenizer
  ([Workspace](workspace.md#planned-crates)), cutting at the operators it finds, so the gate
  classifies words and never parses shell syntax.
- **Installer.** `continual install claude` lives in the same adapter. Writing hook entries
  into a settings file has no domain rule, so it adds no port (W1). It asks the engine, which
  loads the workspace as for every command, for each event's longest run, and writes that as
  the hook's timeout.
- **The gate core.** `continual-gate`, one of Continual's cores
  ([Workspace](workspace.md#planned-crates)), classifies a command's segments as a gated
  action, a bypass or neither, from the built-in and configured recognisers and the approval
  label it is given, and returns a gate decision. It keeps no state: the same command under
  the same rules always gets the same answer. It knows nothing of pipelines or checks. The
  composition root reads the approval label from the guard core, which defines it, and gives
  it to the gate when it builds it.
- **`GateRules` port.** The gate asks for the repository's review-gate settings and its
  blocked commands. The `gate-engine` bridge answers by asking the engine, which keeps
  `[review_gate]` and `[[blocked_commands]]` as located values
  ([Pipeline](pipeline.md#the-engine-core)). The bridge reads the located values into the
  gate's settings. When the configuration cannot be loaded or pins another version, the port
  answers with that error, the gate's decision fails with it, and the driving adapter blocks
  the command with the reason (E7).
- **No state between hook calls.** The gate's answer needs none. The one behaviour that spans
  two calls, letting a second stop through, reads the field of Claude Code's `Stop` payload
  that says the agent is already continuing from a blocked stop.
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
- **The adapter splits shell commands; the gate classifies them.** Shell syntax is a format,
  translated at the edge (A1). Which commands are gated or bypasses is a rule.
- **The gate asks the engine for the repository's rules, through one port.** The gate decides
  about commands, and needs the configuration for its two sections only. The engine stays the one
  reader of `continual.toml`, so the pinned version and the file's shape are checked in one
  place. Rejected: a configuration port of the gate's own on the `toml-config` adapter, which
  would check the pinned version a second time or not at all.
- **A gate that cannot answer blocks, a broken configuration included.** A hook that fails
  must not let a bypass through, so every failure it can catch becomes a block. A
  configuration that cannot be loaded is one of those failures: every other command refuses
  to run on it, and a gate that kept answering would do so without the repository's own
  blocked commands and recognisers, which an agent could then shed by breaking the file. The
  agent is not locked out of the repair, since it edits the file with tools the gate does not
  cover. Rejected: falling back to the built-in rules alone; and falling back to the base
  revision's configuration, as the guard does, which answers with rules the working tree no
  longer states and asks git before every command.
- **Commands are matched by their words, not by the files they write.** Recognising every
  command that could rewrite a hook file needs shell analysis Continual does not do; the
  guard reports changes to those files instead.
- **The approval label is always blocked, in every repository.** It is one fixed name, not a
  setting, so the gate blocks it as a built-in bypass: there is nothing to configure, nothing
  to forget in `[[blocked_commands]]`, and it stays blocked when a repository lists no guard
  in any step. Why the name is fixed is in
  [Built-in checks](built-in-checks.md#decisions). The name is written once, in the guard
  core, and the composition root hands it to the gate, so neither core knows the other.
  Rejected: carrying the label to the gate as a word the configured checks reserve, through
  the registry, the engine and a port, which only a configurable label needs and which
  blocks nothing once a change removes the guard entry.
- **The turn-end checks run in full before the turn ends.** The agent hands back to the
  contributor only with checked work, at the cost of slower turns. Rejected: running them in
  the background, and cutting them off at a short hook timeout, either of which lets a turn end
  on unchecked work.
- **A hook's timeout is a copy that the contributor keeps up to date.** Claude Code applies
  the timeout and reads it only from its own settings, so the number has to be written
  there, and `continual install claude` is the one command that writes it. Running it again
  after the configuration changes is a documented duty
  ([keeping the mod up to date](#keeping-the-mod-up-to-date)), not something Continual
  detects. Rejected: comparing the installed timeout with the configuration on every hook
  call; a step that checks the settings in a pipeline; one fixed timeout for every
  repository; and a deadline for the whole run. Each adds machinery for a change that is
  rare and made on purpose.
- **The gate keeps no state.** Each command is answered from its words and the configuration
  alone. There is then no record of an approval that a command the agent runs could write, no
  port and no adapter to keep one, and one answer for one command. A repository that wants
  fewer questions leaves an action out of `ask`, and what the harness remembers about an
  approval is the harness's own business, as a step's state is its tool's. Rejected: waves, in
  which approving one commit or push lets the next ones through until the contributor's next
  message, and allowances a contributor gives for a session with `/allow`. Both need state
  that outlives a hook call, kept where the agent it gates can reach it.
- **Continual never answers `allow`.** It answers `pass` to a command it has no objection to,
  so Claude Code's permission rules decide, with the deny and ask rules the contributor keeps
  there. Rejected: answering `allow`, which would skip Claude Code's prompt for commands
  nobody approved.
