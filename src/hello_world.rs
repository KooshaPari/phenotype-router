//! Hello-world port fixture.
//!
//! Used as a baseline in parity tests against the real Bifrost adapter
//! (per ADR-050 § "Hello-world port + bifrost lib wrapper").

use crate::decision::{Decision, Request, Response};

/// Response from a [`HelloWorldPort`] invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelloWorldResponse {
    /// Echo of the request id.
    pub id: String,
    /// Echo of the request payload.
    pub payload: String,
    /// Fixed decision for the hello-world fixture (always `Allow`).
    pub decision: Decision,
}

/// The hello-world port contract.
///
/// The default impl echoes the request id and payload through, returning a
/// fixed `Allow` decision. Adapters can compare their output to this
/// shape to verify wire-format parity.
pub trait HelloWorldPort: Send + Sync {
    /// Invoke the hello-world port.
    fn hello(&self, req: &Request) -> HelloWorldResponse;
}

/// Default hello-world port used as a fixture in parity tests.
#[derive(Debug, Default, Clone, Copy)]
pub struct HelloWorld;

impl HelloWorldPort for HelloWorld {
    fn hello(&self, req: &Request) -> HelloWorldResponse {
        HelloWorldResponse {
            id: req.id.clone(),
            payload: req.payload.clone(),
            decision: Decision::Allow,
        }
    }
}

/// Helper for callers that need a [`Response`] view of a hello-world
/// invocation.
pub fn hello_response(req: &Request) -> Response {
    let hw = HelloWorld.hello(req);
    Response {
        decision: hw.decision,
        trace: vec![
            ("router.port".to_string(), "hello-world".to_string()),
            ("router.id".to_string(), hw.id),
            ("router.payload".to_string(), hw.payload),
        ],
    }
}
