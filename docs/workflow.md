# Workflow

How a change to Continual goes from an idea to a merged pull request, for a contributor working
alone or with an agent. It covers where the design is written, how work is tracked, the rules
agents follow, the review gate and the hooks that hold it, and Tusk, which indexes all of it.
The rules are in [CONVENTIONS.md](../CONVENTIONS.md): the design-document and ticket rules
(K), the process rules (G) and the review rules (R).

## Concepts

- **Contributor**: anyone working on Continual, with or without an agent. A contributor working
  with an agent reviews its changes, decides when to commit, push or open a pull request, and
  approves guarded changes.
- **Design document**: a living document in `docs/` that describes one area of Continual as a
  whole, both what is built and what is planned.
- **Ticket**: one unit of planned work, a Tusk `ticket` node in `tickets/`.
- **Points**: a ticket's relative effort on the Fibonacci scale (1, 2, 3, 5, 8, 13, 21).
- **Task**: a ticket of 1, 2 or 3 points, built in one pull request on top of what is merged.
- **Story**: work of 5 points or more, as a parent ticket split into tasks.
- **Epic**: a parent ticket whose children are stories, making one milestone.
- **Planned marker**: the line `> Planned: [<ticket title>](../tickets/<path>.md)` placed where
  a design document describes something not built yet.
- **Wave**: the stretch after a contributor approves a commit or push, in which later commits
  and pushes run without asking until the contributor's next message.

## Behaviour

### From idea to merged change

1. **Design.** The design goes into the design documents of the areas it touches, in the
   present tense, with a planned marker on each part not built yet. The work is estimated in
   points and split into tasks of 1, 2 or 3 points, grouped under stories and epics, each
   created as `pending`. Both are reviewed like any other change, before or together with the
   work.
2. **Work.** The ticket goes to `active`. The agent names the placement step, writes the
   failing test first, and keeps the diff within the layers it named.
3. **Ready.** `cargo xtask ci` passes. The agent says in one line that the change is ready and
   stops (the review gate).
4. **Pull request.** On the contributor's instruction, the change is committed, pushed and
   opened as a pull request. By then the design documents describe the change, the markers it
   fulfils are gone, its tickets are `completed`, any work left for later is a new ticket
   listed under "Follow-ups", and what came up while building is in the tickets that will meet
   it or under the documents' open questions (K10).

Work that is dropped loses its ticket and its planned markers in the same change.

### Tickets

- A ticket is created with `status: pending`. The workflow from Tusk's `kanban` pack refuses
  any other starting state.
- Through Tusk, the status moves from `pending` to `active` and from `active` to `completed`.
  It can go back from `active` to `pending`, and a `completed` ticket can be reopened as
  `pending`. Tusk refuses any other move.
- A child ticket names its parent with `parent`, and `blocks` orders tickets that depend on
  each other. Tusk refuses cycles in both.
- Every ticket with no children is a task with `points` of 1, 2 or 3. Work estimated at 5 or
  more becomes a story, split into tasks, and carries no points itself; its size is the sum of
  its tasks. Each task is workable on its own, as K9 defines it. `tickets/README.md` gives a
  guide to what each size usually holds.
- Every agent session starts with the vision, the memory note and the tasks that are ready to
  start (the `ready-tickets` alias): the tasks that no open ticket blocks, on the task itself
  or on its story. The `open-tickets` alias lists every ticket not yet completed.
- `tusk doctor` reports a ticket outside `tickets/`, a file in `tickets/` that is not a
  ticket, a spec or plan under `docs/`, and a link to a file that no longer exists.

### The review gate

The full rule is R1. In practice:

- When a change is ready, the agent says so in one line and stops. It never pastes diffs or
  file contents into chat. The contributor reviews in their editor, and their comments arrive
  as ordinary messages, or on the pull request once it exists. Comments are not approval.
- Claude Code asks the contributor before every pull-request creation or merge and before the
  first commit or push after each of their messages, whatever form the command takes. Approving
  that commit or push opens a wave. Inside a wave, pull requests still ask and bypasses are
  still blocked.
- These commands are always blocked: `--no-verify` on a commit or a push, `-n` on a commit,
  changes to `core.hooksPath`, `LEFTHOOK=0` or `LEFTHOOK_EXCLUDE`, `lefthook uninstall`, force
  pushes, and any command naming the guard approval label.

## Design

### Where each kind of knowledge lives

| Home | Holds |
| --- | --- |
| `VISION.md` | What Continual does for the people and agents using it. |
| `CONVENTIONS.md` | The rules every change follows, self-contained. |
| `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, the `placing-a-change` skill | Short extracts of `CONVENTIONS.md` for agents. |
| `docs/` | How each area is built and why, one design document per area. |
| `tickets/` | Planned work. |
| `MEMORY.md` | What one session must hand to the next and no other home holds. |
| Doc comments | Each item's contract: inputs, outputs, errors, invariants. |
| `README.md` | Setup and commands. |
| Git history and pull requests | What changed, when and why. |

A fact has one home, and every other place links to it (K8).

Each task is built in a session of its own, and a session's context ends with it. So a session
leaves nothing the next one needs in its own context: what it learned goes to its home above,
and what has no home goes to `MEMORY.md`, a short note that the Tusk digest hands to every
session. The note is rewritten whole, like a design document: the part that changed is
rewritten, what no longer holds is deleted, and it never grows into a log of sessions.

### Design documents

`docs/README.md` lists the documents and gives the outline every document follows: an opening
paragraph on what the area does and for whom, then Concepts, Behaviour, Design, Decisions and
Open questions, leaving out a section with nothing to say. A document has no date, status
line or change history.

What comes up while a task is built is tracked where the next task reads it, never left in a
session (K10). A nuance the design does not spell out, or something it leaves ambiguous, goes
into the open ticket that will meet it next, under "Carried over". What the design leaves
undecided is also written under Open questions in the area's document, with the ticket that
will decide it when one exists, and that ticket links to it. Work it calls for becomes a
ticket. The agent tells the contributor of each when the change is ready. A gap the task
cannot finish without is a stop-and-report instead (G7).

### Tickets

Tickets are markdown files with Tusk frontmatter (`type: ticket`, `title`, `status`,
`points` on a task, and optionally `priority`, `order`, `parent` and `blocks`). Each epic has a
folder holding `epic.md` and one numbered folder per story, which holds `story.md` and the
story's tasks, so the folders show the work as the `parent` edges arrange it.
`tickets/README.md` holds the layout, the template and the Tusk calls that create and move
them. A ticket links to the design-document sections that hold
its design and does not repeat them.

### Agent rules

- `CONVENTIONS.md` is the complete rule set, written for every contributor, human or agent,
  and self-contained. It keeps the rule IDs of the research doc,
  ["Clean ports and adapters agent rules"](https://claude.ai/artifact/Ks8VYF57ui3Buoy9nvaMof),
  so each rule traces back to its reasons and sources. Where a tool checks a rule, the tool's
  configuration is the source of truth.
- `AGENTS.md` is the short form, under about 100 lines: commands, paths, the hard invariants,
  one line per tool-checked rule, and a single `IMPORTANT`, on the review gate (D18).
  `CLAUDE.md` imports it and adds what is specific to Claude Code.
- The path-scoped rules in `.claude/rules/` load when an agent touches matching files:
  `core.md` for the cores, `adapters.md` for adapters and binaries, `tests.md` for tests and
  `docs.md` for `docs/` and `tickets/`.
- The `placing-a-change` skill holds the placement procedure with worked examples, and the
  workflows for adding a driven port and for swapping a technology.

### Claude Code settings and hooks

`.claude/settings.json` asks before `gh pr create` and `gh pr merge` and runs four hooks. They
need `jq`. Without it, the Bash guard blocks every command, and the edit and stop hooks warn
without blocking.

- `guard-bash.sh` (before every Bash command) blocks the bypasses listed above and asks for the
  review gate's approvals. If the hook itself fails, the command is blocked.
- `review-wave.sh` (after Bash commands, and on each contributor message) opens a wave when an
  approved commit or push has run, and closes it on the contributor's next message. The wave
  is a file under `$TMPDIR/continual-review-wave/`, one per session.
- `after-edit.sh` (after Edit and Write) formats an edited Rust file and runs the comments
  check on it.
- `stop-checks.sh` (when the agent stops) runs fmt, clippy, comments, arch and nextest once if
  tracked Rust or TOML files have uncommitted changes. A second stop in the same turn goes
  through, so the agent can stop and report a failure.

> Planned: [Check Continual with a pinned release](../tickets/first-release/7-self-hosting/story.md)

These scripts are replaced by Continual's own [harness mod](harness.md), installed with
`continual install claude` and configured in `continual.toml`, once a pinned release holds
the review gate. The mod keeps no state between hook calls, so waves end with the scripts:
every commit and push then asks ([Harness](harness.md#the-review-gate)).

### Tusk

- `tusk.toml` is the committed manifest. The index in `.tusk/` is not committed.
- Tusk indexes `VISION.md`, `CONVENTIONS.md`, `MEMORY.md`, `docs/` and `tickets/`. It skips build output,
  tooling and the entry files agents load directly (`AGENTS.md`, `CLAUDE.md` and every
  `README.md`).
- The `vault` pack makes untyped markdown a `note`. The `kanban` pack adds the `ticket` type,
  the `parent` and `blocks` edges, and the status workflow.
- The `links-to` edge records each workspace path a page links to or names in inline code.
  `tusk doctor` warns when a named file is gone, and `names-path=tickets/<path>.md*` finds the
  planned markers that point at a ticket.
- The rules `ticket-location`, `tickets-folder` and `no-specs-or-plans` turn K1 and K5 into
  `tusk doctor` errors. `task-size` and `parent-points` do the same for K9, through the
  `points` property the manifest adds to tickets.
- Semantic queries use Ollama with `nomic-embed-text`, with the query and document prefixes and
  the lexical weight of 0.5 taken from Tusk's own measured settings. Without Ollama, structural
  queries still work and semantic ones fail.
- The session digest pins `VISION.md` and `MEMORY.md` and includes the `ready-tickets` alias,
  within 9500 bytes.
- `tusk claude install` writes the Tusk mod to `.claude/skills/tusk/`: the MCP server, a
  session-start hook that injects the digest, and an after-edit hook that lists the pages
  naming an edited file. It also approves the MCP server in `.claude/settings.local.json`.

### Pull requests

`.github/pull_request_template.md` has sections for the summary, the motivation (the ticket or
the behaviour in `VISION.md`), the placement step and layers touched, implementation details,
design decisions and rule departures, tests added, evidence, the definition-of-done checklist
and follow-ups. Rules R3 to R5 govern titles, descriptions and attribution.

## Decisions

- **Design documents instead of specs and plans.** A spec describes one change at one moment
  and goes stale once the change is built, and a plan is spent once it runs. A design document
  stays true because every change updates it, so a reader finds an area's whole design, built
  and planned, in one place.
- **Planned design lives in the design documents, marked.** The design is reviewed before it
  is built without a separate spec, and the marker ties each unbuilt part to its ticket.
  Rejected: keeping unbuilt design in ticket bodies, which splits an area's design across
  files.
- **Tickets are Tusk nodes in the repository.** They are versioned and reviewed with the code
  they plan, and agents query them with the same tools as the documents.
- **The `kanban` pack as shipped.** Its three states cover the work so far. Dropped work is
  deleted rather than given a state of its own. The manifest adds only the `points` property
  and the K9 rules.
- **Folders follow the epics and stories.** A contributor sees a milestone's stories, their
  build order and their tasks by browsing `tickets/`. Story folders are named for their area
  and numbered rather than named for the outcome, so paths stay short. Rejected: one flat
  folder, which hides the hierarchy among dozens of files; and folders named for each parent's
  full outcome, which makes long paths.
- **Tasks of at most 3 points, each workable on its own.** Each task adds one tested slice of
  the hexagon, such as a core rule, a port with its fake or an adapter, on top of what is
  merged, so every later change builds on code that already meets the definition of done. More
  pull requests cost more at the start, but the cost of each new feature levels off as the
  ports and adapters it needs already exist, while small reviews keep quality and standards
  high. Rejected: one ticket per feature, which makes pull requests too large to review
  closely; estimates in hours, which vary with who or which agent does the work; and points on
  stories, which drift from the sum of their tasks.
- **The review gate lives in hooks as well as in prose**, so it holds when an agent forgets the
  rule. Waves keep it to one question per contributor message.
- **What sessions hand on lives in the vault, in one short note.** Each task gets a session of
  its own, so what a session learned has to outlive it in a file that every contributor and
  every agent reads, and the digest delivers that file at the start of each session.
  Rejected: an agent's private memory, which other contributors and other harnesses never
  see; and a log with an entry for each session, which grows until nobody reads it.
- **The digest lists the tasks that are ready to start, not every open ticket.** A session
  builds one task, so what it needs to see is what can start now, and that list stays short
  however long the plan is. Rejected: listing every open ticket, which is in alphabetical
  order, not build order, and was cut off at the digest's size limit.
- **The stop hook does not run the guard.** A Stop hook can show output only by blocking, and
  guard findings must never block. The guard runs in `cargo xtask ci`, pre-push and CI.
- **The research doc is linked, not copied**, so there is one live version. `CONVENTIONS.md`
  stands on its own for anyone who cannot open it.
- **The generated Tusk mod is not committed.** Every Tusk upgrade rewrites it, and each
  contributor runs `tusk claude install` anyway to approve the MCP server.
- **`tusk doctor` is not in CI**, since CI would need Tusk and Ollama installed.
