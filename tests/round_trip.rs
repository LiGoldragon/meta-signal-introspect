use meta_signal_introspect::{
    ConfigurationGeneration, ConfigurationRejected, ConfigurationRejectionReason, Configured,
    IntrospectDaemonConfiguration, Operation, RequestUnimplemented, UnimplementedReason,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};
use signal_persona::origin::{OwnerIdentity, UnixUserIdentifier};
use signal_persona::{SocketMode, WirePath};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn configuration() -> IntrospectDaemonConfiguration {
    IntrospectDaemonConfiguration {
        introspect_socket_path: WirePath::new("/run/persona/introspect.sock"),
        introspect_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/introspect-meta.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/introspect.sema"),
        manager_socket_path: WirePath::new(""),
        router_socket_path: WirePath::new("/run/persona/router.sock"),
        terminal_socket_path: WirePath::new(""),
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
    }
}

fn round_trip_request(request: Operation) -> Operation {
    let frame = meta_signal_introspect::Frame::new(meta_signal_introspect::FrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode request");
    let decoded = meta_signal_introspect::Frame::decode_length_prefixed(&bytes)
        .expect("decode request frame");
    match decoded.into_body() {
        meta_signal_introspect::FrameBody::Request { request, .. } => {
            request.payloads().head().clone()
        }
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(
    reply: meta_signal_introspect::MetaIntrospectReply,
) -> meta_signal_introspect::MetaIntrospectReply {
    let frame = meta_signal_introspect::Frame::new(meta_signal_introspect::FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded =
        meta_signal_introspect::Frame::decode_length_prefixed(&bytes).expect("decode reply frame");
    match decoded.into_body() {
        meta_signal_introspect::FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

#[test]
fn configure_request_round_trips_through_length_prefixed_frame() {
    let request = Operation::Configure(configuration());
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn meta_introspect_request_heads_are_contract_local_operations() {
    assert_eq!(<Operation as SignalOperationHeads>::HEADS, &["Configure"]);
}

#[test]
fn meta_introspect_replies_round_trip_through_length_prefixed_frame() {
    let replies = [
        meta_signal_introspect::MetaIntrospectReply::Configured(Configured {
            generation: ConfigurationGeneration::new(3),
        }),
        meta_signal_introspect::MetaIntrospectReply::ConfigurationRejected(ConfigurationRejected {
            reason: ConfigurationRejectionReason::UnknownPeerComponent,
        }),
        meta_signal_introspect::MetaIntrospectReply::RequestUnimplemented(RequestUnimplemented {
            operation: meta_signal_introspect::OperationKind::Configure,
            reason: UnimplementedReason::NotBuiltYet,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn meta_introspect_operations_encode_as_contract_local_nota_heads() {
    use nota_next::{NotaEncode, NotaSource};

    let request = Operation::Configure(configuration());
    let text = request.to_nota();
    assert!(text.starts_with("(Configure"));
    let decoded = NotaSource::new(&text)
        .parse::<Operation>()
        .expect("decode request nota");
    assert_eq!(decoded, request);
}
