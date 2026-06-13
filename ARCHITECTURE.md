# meta-signal-introspect — architecture

Meta policy Signal contract for privileged `introspect` daemon configuration.

## Surface

This crate owns the meta channel for `introspect`:

- request: `Configure(IntrospectDaemonConfiguration)`;
- replies: `Configured`, `ConfigurationRejected`, `RequestUnimplemented`;
- the typed configuration generation and rejection/unimplemented reason enums.

`IntrospectDaemonConfiguration` is imported from `signal-introspect`. The same
record is used for the binary daemon startup file and later meta-plane
configuration traffic.

## Boundaries

This crate carries only wire vocabulary and codecs. It does not own:

- the `introspect` daemon runtime;
- socket binding;
- peer reachability checks;
- hot-configuration reduction;
- the `introspect.sema` store;
- ordinary introspection query traffic.

Ordinary query and observation traffic lives in `signal-introspect`. Runtime
actors, storage, peer fan-out, and CLI behavior live in `introspect`.

## Constraints

- The meta operation is a contract-local `Configure` root, not a public Sema
  class wrapper.
- Configuration is typed and binary on the daemon boundary; inline NOTA remains
  a client/authoring surface.
- Default builds currently retain the older NOTA-enabled contract shape; the
  destination is binary-by-default with `nota-text` as an explicit edge feature
  when this crate migrates to schema-derived output.
- All request and reply variants need frame round-trip witnesses.

## Code Map

```text
src/lib.rs          handwritten meta contract surface
tests/round_trip.rs frame and NOTA witnesses
INTENT.md           repo-scope intent
```

