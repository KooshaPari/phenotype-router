# AGENTS.md — `phenotype-router`

> AI-agent context for the `phenotype-router` substrate. Read this
> before making any code changes. Mirror the conventions in
> `pheno-port-adapter/AGENTS.md` and `pheno-tracing/AGENTS.md`.

## Substrate role

`phenotype-router` is the **decision layer** of the Phenotype router
architecture (ADR-050 / ADR-051, §8 router-architecture ACCEPTED
2026-06-20). It is a hexagonal L4 Port/Adapter primitive
(ADR-038) with OTel-native OTLP span emission (ADR-012 / ADR-036B).

Per ADR-023 Rule 3, `phenotype-router` is a `pheno-*-lib` — pure
reusable Rust library, single concern (decision layer), single crate.
The `otlp` feature (on by default) wires every decision into
`OtlpDecisionRecorder` so spans flow through a consumer-wired
`TracePort`.

## Repo layout

```
phenotype-router/
├── Cargo.toml              # standalone crate
├── SPEC.md                 # 1-page spec (ADR-023 Rule 3.1)
├── README.md               # public-facing docs
├── llms.txt                # LLM context (mirror this file)
├── WORKLOG.md              # v2.1 schema with device: field
├── CHANGELOG.md            # cliff-style
├── LICENSE-MIT             # MIT
├── AGENTS.md               # this file
├── CODEOWNERS              # @KooshaPari owns all
├── flake.nix               # nix dev shell (ADR-039)
├── .github/workflows/ci.yml
├── src/
│   ├── lib.rs              # public exports
│   ├── decision.rs         # DecisionLayer port
│   ├── bifrost_adapter.rs  # BifrostAdapter stub
│   ├── hello_world.rs      # HelloWorld fixture
│   ├── otel.rs             # OtlpDecisionRecorder + TracePort
│   ├── tracing.rs          # tracing façade + DECISION_SPAN_NAME
│   ├── chaos.rs            # ADR-052 §4 fault-injection framework
│   ├── sdk.rs              # LlmPort / DecisionPlugin / ConnectorPort
│   └── plugins/            # v13 3-plugin port wave
├── tests/                  # 4 integration test files
├── benches/decision.rs     # criterion bench
└── docs/
    ├── concept.md          # concept doc
    ├── architecture.md     # architecture notes
    └── perf-budget.md      # perf budget table
```

## Conventions

- **Branch naming:** `chore/<req-id>-<slug>-<date>` for chore work;
  `feat/<req-id>-<slug>-<date>` for features.
- **Commit messages:** Conventional Commits (`feat:`, `fix:`,
  `chore:`, `docs:`, `refactor:`, `test:`, `build:`, `ci:`) with
  optional scope.
- **PR labels:** `governance` for cleanup, `L<n>-#<n>` for tracking
  against DAG level.
- **SOTA artifacts:** `findings/`, `plans/`, `worklogs/`, `docs/adr/`.
- **Worklog schema:** v2.1 (ADR-015 v2.1) with the `device:` field
  (one of `macbook | heavy-runner | subagent | ci`). The MacBook is
  not a heavy-work device; v0.3.0 bootstrap work all ran on
  `device: macbook`.

## Quality bar (ADR-023 Rule 3.1)

Every change must keep the substrate quality bar green:

1. **Spec:** `SPEC.md` (1 page).
2. **Docs:** `README.md` + `docs/concept.md` (5-line quickstart +
   when / when-not).
3. **Tests:** unit + integ + e2e + perf + chaos (this is a Tier-1
   fleet-critical substrate per ADR-040).
4. **Observability:** OTLP span emission via `OtlpDecisionRecorder`.
5. **Coverage:** 60 % line coverage (federated-service tier,
   ADR-040); enforced by `llvm-cov.toml` + CI.
6. **CI:** `.github/workflows/ci.yml` runs build + test + chaos +
   bench + clippy.
7. **Worklog:** `WORKLOG.md` v2.1 with `device:` field.
8. **CHANGELOG:** keep `CHANGELOG.md` current on every PR.

## When working here

- **Don't add a path dep on `pheno-tracing`.** The substrate uses an
  in-tree `TracePort` so the OTel bridge works without a fleet-wide
  circular dep. Consumers wire their own OTel SDK via a one-line
  shim. (The `pheno-tracing` crate has a pre-existing v14 cycle-4
  T7 compile error referencing a non-existent
  `pheno_otel::metrics` module; tracked as v15 P2 follow-up.)
- **Don't add the `chaos` module without `--features chaos`.** The
  `chaos` feature gates `rand`; prod builds do not pay the dep
  cost.
- **Don't disable the `otlp` feature by default.** ADR-023 Rule 3.1
  requires observability substrate adoption.
- **Don't add a new public type without a unit test.** The hexagonal
  contract (`name` / `adapter_kind` / `health` / `decide`) is
  the substrate's surface area; every new port/adapter must be
  tested for all four methods.
- **Don't change the `phenotype.router.decision` span name.** It's
  pinned by ADR-052 §3; changing it breaks fleet-wide observability.

## Related ADRs

- **ADR-023** (agent-effort governance — substrate placement) —
  `docs/adr/2026-06-15/ADR-023-agent-effort-governance.md`.
- **ADR-038** (hexagonal L4 Port trait formal policy) —
  `docs/adr/2026-06-18/ADR-038-hexagonal-port-adapter-l4-policy.md`.
- **ADR-040** (test coverage gates per tier) —
  `docs/adr/2026-06-18/ADR-040-test-coverage-gates-per-tier.md`.
- **ADR-042B** (substrate quality bar formal) —
  `docs/adr/2026-06-18/ADR-042-substrate-quality-bar.md`.
- **ADR-050** (router rebuild — Phenotype-owned decision layer) —
  `docs/adr/2026-06-20/ADR-050-router-rebuild.md`.
- **ADR-051** (Bifrost-as-library) —
  `docs/adr/2026-06-20/ADR-051-bifrost-as-library.md`.
- **ADR-052** (plugin SDK spec) —
  `docs/adr/2026-06-20/ADR-052-plugin-sdk-spec.md`.
- **ADR-012 / ADR-036B** (pheno-tracing substrate canonical) —
  `docs/adr/2026-06-15/ADR-012-pheno-tracing-canonical.md` and
  `docs/adr/2026-06-18/ADR-036-pheno-tracing-substrate-canonical.md`.

## Related substrate crates

- [`pheno-port-adapter`](../pheno-port-adapter/) — hexagonal L4 reference impl.
- [`pheno-tracing`](../pheno-tracing/) — tracing substrate (v15 follow-up).
- [`pheno-otel`](../pheno-otel/) — OTLP wire-format exporter.
- [`pheno-config`](../pheno-config/) — config substrate (12-factor cascade).
- [`pheno-worklog-schema`](../pheno-worklog-schema/) — worklog v2.1 parser.
