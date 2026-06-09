# INTENT — meta-signal-introspect

*The meta-only wire contract for privileged `introspect` daemon configuration.
Companion to `Cargo.toml` and the ordinary `signal-introspect` contract.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is for the `meta-signal-introspect`
contract. Workspace-shape intent stays in `primary/INTENT.md`; the component
daemon intent stays in `introspect/INTENT.md`; ordinary introspection query and
observation traffic stays in `signal-introspect/INTENT.md`.

## Why this repo exists

Every Persona component has exactly two contracts: `signal-<component>`
(ordinary) and `meta-signal-<component>` (meta). `meta-signal-introspect` is
the second leg for `introspect` — the authority surface that configures the
`introspect-daemon`, including the peer-daemon set the inspection plane fans
out to and its own `introspect.sema` location. Before this repo, `introspect`
had only its ordinary contract; this completes the pair.

## The channel shape

The meta plane's baseline content is daemon configuration. The channel carries
a single `Configure` operation whose payload is the typed
`IntrospectDaemonConfiguration` imported from `signal-introspect` — the same
record that is the daemon's binary startup message. Reconfiguration arrives
over this meta plane as the same typed record, never as flags.

- **Request:** `Configure(IntrospectDaemonConfiguration)`.
- **Replies:** `Configured`, `ConfigurationRejected` (typed reason, including
  `UnknownPeerComponent`), `RequestUnimplemented`.

Peer-daemon registration is daemon configuration and so lives inside the
`Configure` payload rather than as bespoke operations.
