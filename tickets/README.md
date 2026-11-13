# Tickets

Each file here is one unit of planned work: a Tusk `ticket` node. The design the work builds
lives in the [design documents](../docs/README.md), and a ticket links to it instead of
repeating it. The rules are K4 to K7 and K9 in
[CONVENTIONS.md](../CONVENTIONS.md#design-documents-and-tickets-k).

## A ticket

A ticket's id is its path without `.md`. A ticket that must land first names the tickets it
holds up with `blocks`.

| Kind | What it is | File | `points` |
| --- | --- | --- | --- |
| Task | One pull request, workable on its own, with no children | Named for its outcome in kebab case, in its story's folder | 1, 2 or 3 |
| Story | Work of 5 points or more, split into tasks that name it with `parent` | `story.md` in a folder named for its area, numbered in build order inside its epic | none; its size is the sum of its tasks |
| Epic | A milestone whose children are stories, ordered with `order` | `epic.md` in a folder named for the milestone | none |

```text
tickets/
├── README.md
├── <task>.md                     a task outside any epic
└── first-release/
    ├── epic.md                   Self-host Continual on its first release
    ├── 1-pipeline/
    │   ├── story.md              Run a repository's steps from continual.toml
    │   ├── validate-step-definitions.md
    │   └── …
    ├── 2-built-in-checks/
    └── …
```

A story's number matches its `order`. When the order changes, rename the folders with
`tusk node move`, which rewrites the `parent` and `blocks` edges that name them; relative links
in the documents and tickets are fixed by hand.

## Points

Estimate on the Fibonacci scale, by effort relative to other work rather than by time. As a
guide:

| Points | Typical task |
| --- | --- |
| 1 | One small change in one layer with its tests, such as a new case in a value object or one more setting an existing adapter reads. |
| 2 | One core rule or type, or one port operation added to an existing port and adapter, with its tests. |
| 3 | A new port with its contract suite, fake and first adapter, or a feature module's first slice in the core. |
| 5 or more | More than one of these, or a change across the core and several adapters: a story. |

A task is workable on its own when it starts from what is merged, needs no other open
ticket's code, and its pull request meets the definition of done by itself. A port arrives
with the first task that uses it, and an adapter with the first task that needs it, so no
task leaves code waiting for the next one.

```markdown
---
type: ticket
title: Check comments in any language
status: pending
priority: medium
points: 2
parent: tickets/first-release/2-built-in-checks/story
blocks:
  - tickets/first-release/2-built-in-checks/run-check-steps
---

# Check comments in any language

## Goal

What the change achieves and for whom, in a paragraph, linking to the behaviour in
VISION.md.

## Design

Links to the design-document sections that hold the design, for example
[Built-in checks: comments](../../../docs/built-in-checks.md#comments), relative to the
ticket's own folder.

## Behaviours to test

- Each behaviour the change adds, named in domain language.
- Existing behaviour that must keep working.

## Done when

- What a reviewer can check to call the work finished.
- The design documents describe the change, and its planned markers are gone.
```

`status` is required, and so is `points` on a task. `priority` (`low`, `medium` or `high`),
`parent`, `blocks` and `order` are optional.

## Status

| Status | Meaning |
| --- | --- |
| `pending` | Planned. Every ticket starts here. |
| `active` | Work has started. |
| `completed` | Finished, set in the pull request that finishes it. |

A ticket can move from `pending` to `active`, from `active` to `completed` or back to
`pending`, and from `completed` back to `pending` when it is reopened. Tusk checks each move
when it writes the file, so change the status through Tusk rather than by hand. Dropped work is
deleted, along with its planned markers in the design documents.

## With Tusk

| To | MCP tool | CLI |
| --- | --- | --- |
| Create a ticket | `tusk_node_create` with `type: "ticket"`, a `title`, `properties: {status: "pending"}` and the body | `tusk node create tickets/<path>.md --type ticket --title "…" --prop status=pending` |
| Change its status | `tusk_node_modify` with `status` | `tusk node modify tickets/<path> --prop status=active` |
| Add a parent or a blocked ticket | `tusk_edge_add` with type `parent` or `blocks` | `tusk edge add --type parent --source tickets/<path> --target tickets/<parent>` |
| List the open tickets | `tusk_run` with alias `open-tickets` | `tusk run open-tickets` |
| List a ticket's children | `tusk_query` with filter `type=ticket AND parent-> id=tickets/<path>` | `tusk query 'type=ticket AND parent-> id=tickets/<path>'` |
| List everything under an epic | `tusk_query` with filter `type=ticket AND parent->+ id=tickets/<epic>` | `tusk query 'type=ticket AND parent->+ id=tickets/<epic>'` |
| Find its planned markers | `tusk_query` with filter `names-path=tickets/<path>.md*` | `tusk query 'names-path=tickets/<path>.md*'` |
| Size a story or an epic | `tusk_query` with filter `type=ticket AND points=* AND parent->+ id=tickets/<story>`, adding the `points` | `tusk query 'type=ticket AND points=* AND parent->+ id=tickets/<story>' --fields id,properties.points` |
| Check the rules | `tusk_doctor` | `tusk doctor` |

The trailing `*` in the marker query makes it a glob, which matches the ticket's own path. A
plain path would also match every page that names the `tickets/` folder.
