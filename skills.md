# skills — meta-signal-introspect

Before editing this repo, read:

- the `ethos` skill — the contract is an ethos file, and the Rust is generated
- the `datom` skill — the text dialect the `datom` feature projects into
- `ARCHITECTURE.md`
- `signal-introspect`'s `ethos/signal.ethos`, which owns the imported nouns

This crate owns only the meta Signal contract for `introspect` configuration.
Do not add runtime, storage, or CLI behavior here.

## Invariants

- The contract is changed by editing `ethos/signal.ethos` and regenerating with
  `ethos-zero`; never by editing `src/generated/signal.rs`.
- Regenerated output is committed in the same change; `build.rs` is the gate.
- `IntrospectDaemonConfiguration` and other shared nouns are imported from
  `signal-introspect`; do not duplicate them.
- Every request and reply variant needs a frame round-trip witness in
  `tests/contract.rs`.
