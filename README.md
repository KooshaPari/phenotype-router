# `phenotype-router`

> Phenotype-owned router **decision layer** — hexagonal L4 Port/Adapter
> primitive with OTel-native OTLP span emission. Substrate for all LLM-routing
> decisions in the Phenotype fleet (ADR-050 / ADR-051, §8 router-architecture
> ACCEPTED 2026-06-20).

## What

A pure-Rust library that maps a `Request` to a `Response` (decision). The
**decision layer** is the boundary where LLM-routing decisions are made.
Every adapter (`BifrostAdapter`, future `SmartFallback`, future
plugin-chain adapters) plugs into the layer via the [`DecisionLayer`] port
trait (hexagonal L4 contract per ADR-038).

## When

- You need a **portable decision layer** for LLM routing in a fleet
  service. Every `decide()` call emits an OTel-compatible span
  (ADR-012 / ADR-036B).
- You want a **substrate-level hexagonal contract** that future
  adapters and plugins can plug into without changing call sites.
- You want **OTel span emission** (no SDK lock-in — the recorder
  produces OTel-compatible `TraceOperation` values that the consumer
  wires to the OTel SDK of their choice).

## When NOT

- You need the **transport layer** (use `phenotype-gateway` —
  Bifrost owns transport, ADR-051).
- You need the **plugin chain runtime** (v0.5.0; v0.3.0 ships the
  decision-layer contract that plugins will plug into).
- You need a **full OTel SDK** with batching, resource attributes, etc.
  — the recorder is a thin span producer; consumers wire the SDK.

## 5-line quickstart

```rust
use phenotype_router::{BifrostAdapter, DecisionLayer, Request};

let adapter = BifrostAdapter::new();
let req = Request::new("user:42", "weather");
let resp = adapter.decide(&req);
assert_eq!(resp.decision.kind_str(), "allow");
```

## Quickstart with OTLP

```rust
use phenotype_router::otel::{OtelConfig, OtlpDecisionRecorder, TracePort};
use phenotype_router::{BifrostAdapter, DecisionLayer, Request};
use std::sync::Arc;

let recorder = OtlpDecisionRecorder::in_memory(OtelConfig::default());
let adapter = BifrostAdapter::new();
let req = Request::new("user:42", "weather");
let resp = adapter.decide(&req);
let op = recorder.build_operation(&adapter, &req, &resp);
recorder.port().submit(op); // or `recorder.record(&adapter, &req, &resp)`
```

See [`docs/concept.md`](docs/concept.md) for the architecture overview.

## Install

```toml
# Cargo.toml
[dependencies]
phenotype-router = { git = "https://github.com/KooshaPari/phenotype-router", branch = "main" }
```

## Quality bar (ADR-023 Rule 3.1)

- **Spec:** [`SPEC.md`](SPEC.md) (1 page).
- **Concept doc:** [`docs/concept.md`](docs/concept.md).
- **Tests:** 56 total — 32 unit + 6 chaos + 6 hello-world + 4 OTLP smoke + 8 e2e.
- **Coverage:** 60 % federated-service tier gate (ADR-040).
- **Observability:** `OtlpDecisionRecorder` (ADR-012 / ADR-036B).
- **CI:** `.github/workflows/ci.yml`.
- **Worklog:** [`WORKLOG.md`](WORKLOG.md) (v2.1 schema).

## License

Dual MIT / Apache-2.0. See [`LICENSE-MIT`](LICENSE-MIT) and
[`LICENSE-APACHE`](LICENSE-APACHE).
