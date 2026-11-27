---
type: ticket
title: Share locations in the first library crate
blocks:
  - tickets/first-release/1-library-crates/build-step-names-on-a-shared-name-rule
  - tickets/first-release/3-built-in-checks/story
  - tickets/first-release/5-claude-code-mod/join-the-gate-to-the-engine
parent: tickets/first-release/1-library-crates/story
points: 2
priority: high
status: pending
---

# Share locations in the first library crate

## Goal

Create the first library crate, `continual-vocabulary`, with `Location` and `Located<T>`
moved out of the engine core, so the next core that speaks of where something is written
uses the same records and no bridge maps them.

## Design

[Workspace: library crates](../../../docs/workspace.md#library-crates), the decisions on them in
[decisions](../../../docs/workspace.md#decisions), and rules H6 to H8 in
[CONVENTIONS.md](../../../CONVENTIONS.md#cores-h).

## Carried over

- The engine re-exports both records from `continual_engine::workspace`, where its tests
  import them today, so no existing test changes (G5).
- `Location`'s doc comment speaks of a configuration file. In the library crate it says what
  every core means by a location, since a finding's is in a source file.
- An agent proposed the crate's name and the contributor has not confirmed it by name
  ([MEMORY.md](../../../MEMORY.md)). Confirm it before creating the crate.
- If [Prepare the crates for crates.io](../7-releases/prepare-the-crates-for-crates-io.md)
  is merged, the new crate meets what that task set up for every product crate.
- Without a `[libraries]` table, `cargo xtask arch` gives a crate under `crates/libraries` no
  role, so the table arrives in the same change as the crate, with a comment like the one
  on `[cores]`. The paragraph on `arch.toml` under "Crates" in `docs/workspace.md` then names
  the library role and its directory beside the others.

## Behaviours to test

- A `Location` prints as `file:line`.
- A configuration problem prints as it does today, `continual.toml:31: message`, and every
  existing test of the engine passes unchanged.
- `cargo xtask arch` passes with the `[libraries]` table, the crate's empty allow-list and the
  engine's dependency on the crate.

## Done when

- The crate lives in `crates/libraries/vocabulary/`, sets `[lints] workspace = true` and
  carries a `clippy.toml` like a core's.
- The engine declares no `Location` or `Located<T>` of its own.
- `Location` and `Located<T>` carry no planned link in `docs/workspace.md`, whose crates table
  and repository layout list the new crate, and `CONVENTIONS.md` and `AGENTS.md` no longer
  say that no library crate exists (guarded changes).
