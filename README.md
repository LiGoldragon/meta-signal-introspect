# meta-signal-introspect

Meta signal contract for privileged introspect daemon configuration.

The meta-only wire contract for `introspect` — the second leg of the two-contract
pair (`signal-introspect` ordinary + `meta-signal-introspect` meta). The meta plane's
baseline content is daemon configuration: a typed `Configure` operation
carrying `introspect`'s `*DaemonConfiguration` (the same record that is the daemon's
binary startup message), with `Configured` / `ConfigurationRejected` /
`RequestUnimplemented` replies.

Default builds carry `nota-text` for CLI/debug projection; the wire is
binary/rkyv. See `ARCHITECTURE.md`.
