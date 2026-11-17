//! The mandatory test for the limit of the first conversation: a server that answers the handshake (`Control.version`) but
//! never answers `capabilities`. `fetch_within` must end with `Error::Timeout`, never wait for ever (`BEFUND-einzelinstanz.md`).
//!
//! Tests: example-based, a real server of the test that stays silent after the handshake.

use std::sync::mpsc;
use std::time::Duration;

use capnp::capability::Rc;
use fs_broadcast_client::{EXPECTED_CONTRACT, Error, Hello, PROGRAM};
use fs_interfaces::streaming_capnp::streaming;
use fs_rpc::{ServerIdentity, run_until_signal_as};

/// Answers nothing: `capabilities` waits for ever; everything else is `unimplemented`.
struct Silent;

impl streaming::Server for Silent {
    async fn capabilities(
        self: Rc<Self>,
        _: streaming::CapabilitiesParams,
        _: streaming::CapabilitiesResults,
    ) -> Result<(), capnp::Error> {
        std::future::pending().await
    }
}

#[test]
fn a_server_that_answers_the_handshake_but_never_the_first_call_ends_with_a_timeout() {
    let dir = tempfile::tempdir().unwrap();
    let socket = dir.path().join("broadcast.sock");
    let path = socket.clone();
    let server = std::thread::spawn(move || {
        let identity = ServerIdentity::new(PROGRAM, env!("CARGO_PKG_VERSION"));
        run_until_signal_as(&path, identity, move || async move {
            Ok(capnp_rpc::new_client::<streaming::Client, _>(Silent).client)
        })
        .unwrap();
    });
    for _ in 0..200 {
        if socket.exists() {
            break;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    // The call runs in its own thread, so a missing limit shows as a failed test and not as a hang.
    let (tx, rx) = mpsc::channel();
    let path = socket.clone();
    std::thread::spawn(move || {
        let nobody = || Err(std::io::Error::other("no starter"));
        let answer = Hello::fetch_within(&path, &nobody, EXPECTED_CONTRACT, Duration::from_secs(2));
        tx.send(answer).ok();
    });
    let answer = rx
        .recv_timeout(Duration::from_secs(20))
        .expect("the first conversation has no limit");
    assert!(
        matches!(answer, Err(Error::Timeout { limit_ms: 2000 })),
        "{answer:?}"
    );
    fs_broadcast_client::stop(&socket).unwrap();
    server.join().unwrap();
}
