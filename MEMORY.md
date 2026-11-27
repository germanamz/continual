# Memory

What one session must hand to the next and no other document holds (K8). Each task is built in
a session of its own, so nothing a later session needs may stay behind in one. Keep this note
short and whole: rewrite the part that changed, delete what no longer holds, and never append
a log of what a session did.

## Reviewing the design

- The contributor reviews the design on a page of diagrams, "Continual Architecture", which
  mirrors the design documents: <https://claude.ai/artifact/TZHbjyiN5dYAhaMcwxYTuz>. They
  settle an open question from an example of the problem, a table of the ways to settle it
  with their costs, and a suggestion, and they answer in comment threads on the page.
- When they ask for the page to follow a change, read the published page, edit that copy and
  publish it to the same link. Its build-order figure quotes each story's tasks and points, so
  recount them when tickets change. An agent can answer or resolve a comment thread only
  after the contributor sends it to Claude.
- An example states its setup before the point it makes, on the page and in the documents.
- An agent filled in some parts of the design from a short answer, and the contributor saw
  them only in a summary: one future for each entry, asynchronous ports down to those that
  read the repository, the `Timer` port, how batches run, and the composition root handing
  the approval label to the gate. A task that finds one of them wrong stops and reports it,
  and need not take it for the contributor's own choice.

## Tickets and Tusk

- After `tusk_edge_add`, check that the edge is in the ticket file's frontmatter.
- `tusk_node_delete` leaves the edges that point at the deleted ticket dangling, so remove
  those `blocks` edges first.
