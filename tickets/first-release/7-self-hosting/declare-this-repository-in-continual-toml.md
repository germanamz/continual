---
type: ticket
title: Declare this repository in continual.toml
blocks: tickets/first-release/7-self-hosting/gate-this-repository-with-the-pinned-release
parent: tickets/first-release/7-self-hosting/story
points: 3
priority: medium
status: pending
---

# Declare this repository in continual.toml

## Goal

Write this repository's root `continual.toml` with every step `cargo xtask ci` runs, and add
the CI dogfood job that runs the working-tree build of `continual` on it with `--unpinned`,
while `cargo xtask` stays the gate.

## Design

[Checks: self-hosting](../../../docs/checks.md#self-hosting).

## Behaviours to test

- `cargo run --package continual -- run ci --unpinned` runs every step `cargo xtask ci` runs,
  in an order the graph allows, and fails on the same fixture violations.
- The dogfood job is required in CI, and `cargo xtask ci` still gates.
- The steps that set `required = false` are named, closing the open question in
  docs/checks.md.

## Done when

- The open question in docs/checks.md is decided and removed.
- The placeholders question in docs/pipeline.md is decided for the steps this file needs.
