//! Router decision port contract (ADR-050).
//!
//! The decision layer is the boundary where a [`Request`] is mapped to a
//! [`Response`]. Adapters (e.g. [`crate::BifrostAdapter`]) implement
//! [`DecisionLayer`] to plug in different decision strategies without
//! changing the call sites.

use thiserror::Error;

/// A request that the decision layer must resolve.
///
/// The shape is intentionally minimal for the hello-world scaffold; richer
/// fields (headers, tracing context, tenant ID) will be added once the
/// Bifrost FFI bridge lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    /// Stable identifier for the request (e.g. tool name, route).
    pub id: String,
    /// Free-form payload the adapter may inspect.
    pub payload: String,
}

/// A decision returned by an adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Allow the request to proceed.
    Allow,
    /// Reject the request with a human-readable reason.
    Deny(String),
}

/// A response returned by an adapter. Wraps a [`Decision`] plus any
/// adapter-specific side-channel data (currently a list of trace fields
/// the call site can attach to a span).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    /// The decision itself.
    pub decision: Decision,
    /// Optional annotations the call site can attach to a span/log line.
    pub trace: Vec<(String, String)>,
}

impl Response {
    /// Construct an `Allow` response with no trace annotations.
    pub fn allow() -> Self {
        Self {
            decision: Decision::Allow,
            trace: Vec::new(),
        }
    }

    /// Construct a `Deny` response with a reason and no trace annotations.
    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            decision: Decision::Deny(reason.into()),
            trace: Vec::new(),
        }
    }
}

/// Errors emitted by the decision layer.
#[derive(Debug, Error)]
pub enum DecisionError {
    /// The adapter could not evaluate the request.
    #[error("adapter error: {0}")]
    Adapter(String),
}

/// The port trait every router decision adapter must implement.
///
/// Implementors must be `Send + Sync` so the layer can be plugged into the
/// fleet-wide concurrency model.
pub trait DecisionLayer: Send + Sync {
    /// Resolve a request into a response.
    fn decide(&self, req: &Request) -> Response;
}
