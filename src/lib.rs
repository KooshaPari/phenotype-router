//! Phenotype router decision layer (ADR-050 / ADR-051, §8 router-architecture
//! ACCEPTED 2026-06-20).
//!
//! This crate owns the **decision layer** of the router architecture. It is
//! a hello-world scaffold with two building blocks:
//!
//! - [`DecisionLayer`] — the port trait every router decision must implement.
//! - [`BifrostAdapter`] — a stub adapter for the upstream Bifrost decision
//!   library (Go-side; absorbed at runtime via the substrate-boundary port
//!   contract defined in ADR-050).
//!
//! Future work: a real FFI bridge to the existing Bifrost Go module in
//! `phenotype-gateway/packages/bifrost/`, plus `HelloWorldPort` for parity
//! tests against the substrate fixture.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod decision;
mod hello_world;
mod bifrost_adapter;

pub use decision::{Decision, DecisionError, DecisionLayer, Request, Response};
pub use hello_world::{hello_response, HelloWorld, HelloWorldPort, HelloWorldResponse};
pub use bifrost_adapter::BifrostAdapter;
