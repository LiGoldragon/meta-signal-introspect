# meta-signal-introspect

Meta Signal contract for privileged `introspect` daemon configuration.

The meta-only wire contract for `introspect` — the second leg of the
two-contract pair (`signal-introspect` ordinary + `meta-signal-introspect`
meta). The meta plane's baseline content is daemon configuration: a typed
`Configure` operation carrying `introspect`'s `IntrospectDaemonConfiguration`
(the same record that is the daemon's binary startup message), with
`Configured` / `ConfigurationRejected` / `RequestUnimplemented` replies.

The contract is declared in `ethos/signal.ethos`; `build.rs` asserts the
committed `src/generated/signal.rs` matches a fresh `ethos-zero` generation.
The wire is binary rkyv. The optional `datom` feature adds the Datom text
projection of every contract type. See `ARCHITECTURE.md`.
