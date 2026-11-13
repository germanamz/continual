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

Write this repository's root `continual.toml` with every step `cargo xtask ci` runs and a `ci`
pipeline that runs them as `xtask` hardcodes today, and add the CI dogfood job that runs the working-tree build of `continual` on it with `--unpinned`,
while `cargo xtask` stays the gate.

## Design

[Checks: self-hosting](../../../docs/checks.md#self-hosting).

## Behaviours to test

- `cargo run --package continual -- run ci --unpinned` runs every step `cargo xtask ci` runs,
  in the same order, and fails on the same fixture violations.
- Every entry of `ci` is required, so the first failure stops the run as `cargo xtask ci` does.
- The dogfood job is required in CI, and `cargo xtask ci` still gates.

## Done when

- The CI inputs the steps need reach them as the flags `cargo xtask` takes today.
