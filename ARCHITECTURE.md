# meta-signal-introspect — architecture

Meta policy Signal contract for privileged `introspect` daemon configuration.

## Direction

This repo is the second leg of the introspect contract pair. Every Persona
component has exactly two contracts: the ordinary `signal-<component>` and the
meta `meta-signal-<component>`. `meta-signal-introspect` is the authority
surface that configures the `introspect` daemon, including the peer-daemon set
the inspection plane fans out to and the daemon's own store location.

Peer-daemon registration is daemon configuration, so it lives inside the
`Configure` payload rather than as bespoke operations. The rejection reason set
therefore includes `UnknownPeerComponent` for a configuration that names a peer
the daemon cannot resolve.

## Surface

`ethos/signal.ethos` is the single source of the contract. Its request root
generates `Query` and its reply root generates `Response`:

| Request | Meaning |
|---|---|
| `Configure(IntrospectDaemonConfiguration)` | Apply the typed daemon configuration. |

| Reply | Meaning |
|---|---|
| `Configured` | The configuration was applied; carries the resulting `ConfigurationGeneration`. |
| `ConfigurationRejected` | Carries a typed `ConfigurationRejectionReason`: `ManagerAuthorityRequired`, `MalformedConfiguration`, `UnknownPeerComponent`. |
| `RequestUnimplemented` | The request reached the meta surface but the runtime path is not built; carries the `MetaIntrospectOperationKind` and a `UnimplementedReason` of `NotBuiltYet` or `DependencyNotReady`. |

`IntrospectDaemonConfiguration` is imported from `signal-introspect`, not
duplicated. The same record is used for the binary daemon startup file and for
later meta-plane configuration traffic.

## Generation

```sh
ethos-zero 'Generate.{ <repo>/ethos/signal.ethos <repo>/src/generated }'
```

`src/generated/signal.rs` is committed. `build.rs` regenerates from the ethos
source at build time and asserts equality with the committed file, so a drifted
generation fails the build rather than the review. No `Datomic` implementation
is hand-written for a declared type.

## Frames

`src/lib.rs` carries the portable frame surface only: `Signal<T>`,
`Signalizable`, `ByteViewable`, and `Restorable<T>`. Archiving is rkyv;
`Signal<T>` carries its target contract in its type so received bytes restore
into the contract they were framed from.

## Boundaries

This crate carries only wire vocabulary and codecs. It does not own:

- the `introspect` daemon runtime;
- socket binding;
- peer reachability checks;
- hot-configuration reduction;
- the introspect store;
- ordinary introspection query traffic.

Ordinary query and observation traffic lives in `signal-introspect`. Runtime
actors, storage, peer fan-out, and CLI behavior live in `introspect`.

## Constraints

| Constraint | Witness |
|---|---|
| The contract shape is declared, never hand-written. | `build.rs` asserts `src/generated/signal.rs` equals a fresh generation from `ethos/signal.ethos`. |
| The meta operation is a contract-local `Configure` root, not a public Sema class wrapper. | The generated `Query` enum has exactly the `Configure` variant. |
| Shared introspect nouns are imported, not copied. | `Query::Configure` carries `signal_introspect::IntrospectDaemonConfiguration`. |
| Every request and reply round-trips over the real wire. | `tests/contract.rs` archives and restores each `Query` and `Response` variant through received bytes, and round-trips `Query` through Datom text under the `datom` feature. |
| Contract code contains no runtime. | Source contains no actors, tokio, storage, or socket implementation. |
| Behavior is homed in traits. | The `no-free-functions` and `no-inherent-methods` Nix checks. |

## Code Map

```text
ethos/signal.ethos      the contract source
build.rs                freshness assertion over the committed generation
src/generated/signal.rs generated contract types
src/lib.rs              portable rkyv Signal frame surface
examples/canonical.datom canonical Datom projections of each request and reply
tests/contract.rs       rkyv frame and Datom witnesses
```
