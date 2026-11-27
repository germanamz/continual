---
type: ticket
title: Publish Continual releases
blocks: tickets/first-release/7-self-hosting/story
order: 6
parent: tickets/first-release/epic
priority: medium
status: pending
---

# Publish Continual releases

## Goal

Make every Continual release installable with `cargo install`, from source, or as a prebuilt
executable for each supported platform, so repositories, including this one, can pin a
release ([humans](../../../VISION.md#humans)).

## Design

[Distribution](../../../docs/distribution.md): every section.

## Behaviours to test

Each child ticket lists the behaviours it adds. List the children with the query in
[tickets/README.md](../../README.md#with-tusk).

## Done when

- Every child ticket is completed.
- A first release exists on crates.io and GitHub.
- The design documents describe the change, and its planned markers are gone.
