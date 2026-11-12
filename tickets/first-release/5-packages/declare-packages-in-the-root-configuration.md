---
type: ticket
title: Declare packages in the root configuration
blocks: tickets/first-release/5-packages/inherit-shared-steps-in-packages
parent: tickets/first-release/5-packages/story
points: 3
priority: medium
status: pending
---

# Declare packages in the root configuration

## Goal

Let a root `continual.toml` list its packages and have each package's own `continual.toml`
read with it, so the workspace knows every package by its name and directory.

## Design

The `[workspace] packages` and `name` bullets in [Pipeline: packages](../../../docs/pipeline.md#packages),
and the `toml-config` notes in [ports and adapters](../../../docs/pipeline.md#ports-and-adapters).

## Behaviours to test

- Every directory a `[workspace] packages` glob matches must hold a `continual.toml` with a
  `name`; a missing file, a missing name and a duplicate name are configuration errors at
  their location.
- A package name follows `CheckName`'s rules.
- Each package's own `[steps]` are read with it.
- Existing: a root configuration without `[workspace]` reads exactly as before.

## Done when

- The `ConfigSource` contract suite covers package configurations, and the adapter passes it.
