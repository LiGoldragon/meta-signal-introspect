//! Meta signal contract — privileged `introspect` daemon configuration.
//!
//! Ordinary introspection query and observation traffic lives in
//! `signal-introspect`. This crate carries the meta plane: the authenticated
//! `Configure` operation that applies `introspect`'s typed daemon
//! configuration (the peer-daemon set the inspection plane fans out to and its
//! own `introspect.sema` location).
//!
//! The basic meta operation of every component is daemon configuration — the
//! `IntrospectDaemonConfiguration` the Persona manager encodes is itself the
//! binary startup message, and later reconfiguration arrives over this meta
//! plane as the same typed record, never as flags.

use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_introspect::IntrospectDaemonConfiguration;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct ConfigurationGeneration(u64);

impl ConfigurationGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl NotaDecode for ConfigurationGeneration {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        Ok(Self(NotaBlock::new(block).parse_integer()?))
    }
}

impl NotaEncode for ConfigurationGeneration {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Configured {
    pub generation: ConfigurationGeneration,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ConfigurationRejectionReason {
    ManagerAuthorityRequired,
    MalformedConfiguration,
    UnknownPeerComponent,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ConfigurationRejected {
    pub reason: ConfigurationRejectionReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestUnimplemented {
    pub operation: OperationKind,
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel MetaIntrospect {
        operation Configure(IntrospectDaemonConfiguration),
    }
    reply MetaIntrospectReply {
        Configured(Configured),
        ConfigurationRejected(ConfigurationRejected),
        RequestUnimplemented(RequestUnimplemented),
    }
}

impl From<IntrospectDaemonConfiguration> for Operation {
    fn from(payload: IntrospectDaemonConfiguration) -> Self {
        Self::Configure(payload)
    }
}
