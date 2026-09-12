use meta_signal_introspect::{
    ByteViewable, ConfigurationRejected, ConfigurationRejectionReason, Configured,
    MetaIntrospectOperationKind, Query, RequestUnimplemented, Response, Restorable, Signal,
    Signalizable, UnimplementedReason,
};
use signal_introspect::IntrospectDaemonConfiguration;
use signal_persona::OwnerIdentity;

fn configuration() -> IntrospectDaemonConfiguration {
    IntrospectDaemonConfiguration {
        introspect_socket_path: "/run/persona/introspect.sock".into(),
        introspect_socket_mode: 0o600,
        supervision_socket_path: "/run/persona/introspect-meta.sock".into(),
        supervision_socket_mode: 0o600,
        store_path: "/var/lib/persona/introspect.sema".into(),
        manager_socket_path: "/run/persona/manager.sock".into(),
        router_socket_path: "/run/persona/router.sock".into(),
        terminal_socket_path: "/run/persona/terminal.sock".into(),
        trace_socket_path: "/run/persona/trace.sock".into(),
        owner_identity: OwnerIdentity::UnixUser(1000),
    }
}

#[test]
fn query_and_response_round_trip_through_received_bytes() {
    let query = Query::Configure(configuration());
    let received =
        Signal::<Query>::from(query.signalize().expect("query archives").bytes().to_vec());
    assert_eq!(received.restore().expect("query restores"), query);

    for response in [
        Response::Configured(Configured {
            configuration_generation: 3,
        }),
        Response::ConfigurationRejected(ConfigurationRejected {
            configuration_rejection_reason: ConfigurationRejectionReason::UnknownPeerComponent,
        }),
        Response::RequestUnimplemented(RequestUnimplemented {
            meta_introspect_operation_kind: MetaIntrospectOperationKind::Configure,
            unimplemented_reason: UnimplementedReason::NotBuiltYet,
        }),
    ] {
        let received = Signal::<Response>::from(
            response
                .signalize()
                .expect("response archives")
                .bytes()
                .to_vec(),
        );
        assert_eq!(received.restore().expect("response restores"), response);
    }
}

#[test]
fn malformed_archive_is_rejected() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
}

#[cfg(feature = "datom")]
#[test]
fn query_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = Query::Configure(configuration());
    let text = query.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<Query>::from(text)
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("Datom restores");
    assert_eq!(restored, query);
}
