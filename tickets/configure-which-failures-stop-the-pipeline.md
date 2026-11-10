---
type: ticket
title: Configure which failures stop the pipeline
priority: medium
status: pending
---

# Configure which failures stop the pipeline

## Goal

Let a repository choose which failing checks stop its pipeline, so one run reports every
failure the repository wants to see together, while the checks it depends on still end the
run. This is the "first configured failure" of the [pipeline](../VISION.md#pipeline).

## Design

[Checks: pipeline failures](../docs/checks.md#pipeline-failures), and its open question in
[Checks: open questions](../docs/checks.md#open-questions).

## Behaviours to test

- A failing step that stops the pipeline ends the run, and the steps after it do not run.
- A failing step that continues reports its failure, and the steps after it run.
- The pipeline fails when any step failed, and names every failed step, in order.
- A step with no setting stops the pipeline.
- The pipeline passes when every step passes.
- Existing: the steps run in the spec order, and the existing pipeline tests pass unchanged.

## Done when

- At least one of Continual's own steps continues after a failure, and docs/checks.md says
  which and why, closing its open question.
- The design documents describe the change, and its planned markers are gone.
