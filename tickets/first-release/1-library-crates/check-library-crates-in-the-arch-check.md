---
type: ticket
title: Check library crates in the arch check
blocks:
  - tickets/first-release/1-library-crates/forbid-traits-in-library-crates
  - tickets/first-release/1-library-crates/hold-library-crates-to-the-cores-coverage
  - tickets/first-release/1-library-crates/share-locations-in-the-first-library-crate
parent: tickets/first-release/1-library-crates/story
points: 3
priority: high
status: pending
---

# Check library crates in the arch check

## Goal

Teach `cargo xtask arch` the library role: a `[libraries]` table in `arch.toml`, the
dependency rules that keep a library crate at the bottom of the workspace, and the source
scan a core gets, so a library crate can arrive with its boundary already held.

## Design

The `arch` bullet in [Checks: library crates](../../../docs/checks.md#library-crates), its
decision in [decisions](../../../docs/checks.md#decisions), and rule H7 in
[CONVENTIONS.md](../../../CONVENTIONS.md#cores-h).

## Carried over

- The workspace has no library crate when this task is built, so `arch.toml` gets no
  `[libraries]` table here and fixtures prove the role. The table arrives with the first
  crate ([Share locations in the first library crate](share-locations-in-the-first-library-crate.md)).
- `CONVENTIONS.md` names `cargo xtask arch` as a check still to come for H6 and H7, and
  describes the check as it is today in the "Check" cells of H1 and H4, in rule B9 and in the
  architecture row of the enforcement map. This task brings those up to date, with the comments
  in `arch.toml` and step 6 of the driven-port workflow in the `placing-a-change` skill. They
  are guarded changes.

## Behaviours to test

- Without a `[libraries]` table, the check reports what it reports today.
- Every crate under the libraries directory is a library crate. A directory that holds no
  crate is a finding, and so are a library crate without an allow-list and a list that names
  no library crate.
- A library crate that depends on a workspace crate is reported, whether that crate is a
  core, an adapter, a composition root, tooling or another library crate, counting normal,
  dev and build dependencies. Its dev-dependency on itself passes.
- A library crate's external dependency off its own allow-list is reported, and an
  allow-list entry without a reason is a finding.
- A core, an adapter and a composition root may each depend on a library crate. A core that
  depends on any other workspace crate is still reported.
- A forbidden `std` path in a library crate's sources is reported as it is in a core's.
- A library crate that does not set `[lints] workspace = true` is reported.

## Done when

- Fixture tests show each behaviour failing and passing, and no existing arch test changes.
- This task's planned marker is gone from `docs/checks.md`, and the `arch` paragraph there
  describes the role as built.
