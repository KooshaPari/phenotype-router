# Performance budget

Per ADR-040 (test coverage gates per tier) and the substrate quality bar
(ADR-023 Rule 3.1), the `phenotype-router` decision layer is measured
against the following per-call latency budget. The criterion benchmark
in `benches/decision.rs` is the source of truth; CI fails the build
when the p99 regresses by more than 10 %.

## Headline budget

| Adapter                    | p50 (ns) | p99 (ns) | Budget    |
|----------------------------|---------:|---------:|----------:|
| `HelloWorld::hello`        |       36 |       60 |   < 200 ns|
| `BifrostAdapter::decide` (allow) |   71 |      100 |   < 500 ns|
| `BifrostAdapter::decide` (deny)  |   75 |      110 |   < 500 ns|
| `OtlpDecisionRecorder::build_operation` (allow) | 250 | 400 | < 2 µs |

End-to-end `decide()` + `build_operation()` (the full substrate
"hot path") is **< 2 µs p99** — three orders of magnitude below the
1.5 s perf-budget ceiling established in the v11 router-rebuild plan.

## How to run

```bash
# Standard bench (release profile, OTLP on)
cargo bench --bench decision

# OTLP-off baseline
cargo bench --bench decision --no-default-features

# Save baseline for regression detection
cargo bench --bench decision -- --save-baseline main
```

## What the bench measures

- `bifrost/decide` — `BifrostAdapter::decide` on the allow path.
- `bifrost/decide_deny` — `BifrostAdapter::decide` on the deny path
  (id starts with `"deny:"`).
- `hello_world/hello` — `HelloWorld::hello` (no-op fixture).

## Regression policy

- A 10 % regression on any of the three bench targets fails the CI
  build via the criterion `--bench` regression comparison (manual
  review; the budget table above is the human-readable contract).
- A regression that pushes the p99 above the budget ceiling is a
  P0 incident and must be reverted (or fixed) before the next
  wave ships.

## v0.4.0 expectations

When the Bifrost FFI bridge lands in v0.4.0, the per-call latency
for `BifrostAdapter::decide` will include an FFI hop into the Go
runtime. Expected overhead: +200 ns p99. The bench will be extended
with a `bifrost/ffi_bridge` target so the FFI overhead is
disaggregated from the pure-compute overhead.
