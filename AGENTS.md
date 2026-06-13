# meta-signal-introspect — Agent Instructions

Read `/home/li/primary/AGENTS.md` first, then
`/home/li/primary/repos/lore/AGENTS.md`.

## Purpose

`meta-signal-introspect` is the meta policy contract for privileged
`introspect` daemon configuration. It is a wire contract crate, not a runtime
component.

## Local Rules

- Use Jujutsu for version control.
- Keep runtime code out of this crate: no actors, sockets, tokio tasks, store
  handles, or filesystem mutation.
- Keep ordinary introspection requests in `signal-introspect`.
- Every contract change needs a frame round-trip witness in tests.
- The daemon configuration type is owned by `signal-introspect`; this crate
  imports it rather than duplicating it.

