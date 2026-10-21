# Architecture

## Shape

The system is a ports-and-adapters Rust workspace with five bounded contexts. `cc-orchestrator` owns the saga; adapters cannot self-promote. OpenFeature, RuVector, and RVF sit behind one anti-corruption boundary each. The evidence engine is deterministic and pure. Policy alone may issue a decision; grants alone authorize one promotion action; recovery is idempotent.

## Runtime flow

1. API/CLI parses validated configuration and a `ReleaseCandidate`.
2. Cohort planner selects the next cohort; OpenFeature evaluates `canary-release-enabled` with cohort context.
3. Exposure ledger records unique subjects; metrics normalize observations.
4. RuVector returns the closest seeded incidents; the adapter checks result identifiers and distance finiteness.
5. E-process consumes each harm indicator without a fixed stopping horizon.
6. Risk engine combines error budget, e-value, and incident penalty; planner proposes an outcome.
7. Policy applies degraded-mode dominance and emits hold/promote/rollback.
8. Promotion issues and consumes a single-use grant. Rollback executes an idempotent recovery plan.
9. Ledger appends events; RVF adapter builds a witness chain; verifier checks report and chain.

## Crate contract map (24 implemented crates)

| Crate | Responsibility | Context | Typed public API | Real callers | ADR/DDD | Tests | Acceptance |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `cc-types` | Shared identifiers, validated percentages and decision enums | Shared Kernel | `ReleaseId, CohortId, Percentage, Decision` | all crates | ADR-01, ADR-03; DDD-01 | value-object invariants and serde roundtrip | Reject invalid exposure and empty identifiers |
| `cc-release` | Release candidate aggregate and digest validation | Release Control | `ReleaseCandidate::new, ReleaseCandidate::activate` | cc-orchestrator | ADR-02, ADR-22; DDD-02 | digest and state-transition tests | Only validated candidates enter planning |
| `cc-cohort` | Ordered cohort plan and exposure ceilings | Release Control | `CohortPlan::next, CohortPlan::current` | cc-orchestrator, cc-policy | ADR-04; DDD-03 | ordering and ceiling tests | No cohort skip or exposure overflow |
| `cc-exposure` | Append-only exposure accounting | Release Control | `ExposureLedger::record, ExposureLedger::total` | cc-orchestrator, cc-risk | ADR-05, ADR-14; DDD-03 | duplicate and monotonicity tests | Every admitted subject counted once |
| `cc-metrics` | Observation validation and normalization | Evidence Intelligence | `Observation::new, ObservationBatch::summary` | cc-eprocess, cc-risk | ADR-06; DDD-04 | range and aggregation tests | Invalid or non-finite metrics fail |
| `cc-openfeature` | Official OpenFeature provider adapter | Release Control | `FlagGate::evaluate` | cc-orchestrator | ADR-07, ADR-19; DDD-05 | real provider evaluation and unknown flag fail-closed | Enterprise SDK executes in workflow |
| `cc-incidents` | Incident aggregate and severity model | Evidence Intelligence | `Incident::new, IncidentSet::dominant` | cc-ruvector, cc-risk | ADR-08; DDD-06 | severity and dominance tests | Historical failures have typed impact |
| `cc-embedding` | Deterministic incident/query embedding | Evidence Intelligence | `IncidentEmbedder::embed` | cc-ruvector | ADR-09; DDD-06 | dimension and stability tests | Same fixture yields same four-vector |
| `cc-ruvector` | RuVector anti-corruption adapter | Evidence Intelligence | `IncidentMemory::open, seed, recall` | cc-orchestrator | ADR-10, ADR-19; DDD-06 | real insert/search and adapter-integrity tests | Nearest incident is source-bound and bounded |
| `cc-eprocess` | Anytime-valid betting wealth state | Evidence Intelligence | `EProcess::observe, evidence_threshold` | cc-risk, cc-evaluation | ADR-11; DDD-07 | null/alternative/optional-stop tests | Wealth remains nonnegative and auditable |
| `cc-risk` | Risk synthesis from live and recalled evidence | Evidence Intelligence | `RiskEngine::assess` | cc-policy, cc-orchestrator | ADR-12; DDD-08 | incident penalty and error-budget tests | Produces explainable bounded risk |
| `cc-planner` | Deterministic promotion/hold/rollback planner | Release Control | `Planner::propose` | cc-policy | ADR-13; DDD-09 | healthy/harmful planning tests | Never promotes without minimum evidence |
| `cc-policy` | Fail-closed policy decision point | Governance | `PolicyEngine::decide` | cc-orchestrator | ADR-14, ADR-19; DDD-09 | dominance matrix tests | All mandatory gates conjunctive |
| `cc-grant` | Single-use rollout grant state machine | Governance | `GrantIssuer::issue, RolloutGrant::consume` | cc-orchestrator | ADR-15; DDD-10 | reuse, expiry and scope tests | Promotion authority is attenuated and one shot |
| `cc-rvf` | RVF witness-chain adapter | Governance | `WitnessChain::seal, WitnessChain::verify` | cc-ledger, cc-verifier | ADR-16, ADR-19; DDD-11 | real chain creation and tamper test | Material events cryptographically chained |
| `cc-ledger` | Hash-linked decision and action ledger | Governance | `AuditLedger::append, entries` | cc-orchestrator, cc-rvf | ADR-17; DDD-11 | ordering and mutation tests | No silent decision replacement |
| `cc-verifier` | Independent report and witness verifier | Governance | `Verifier::verify` | cc-cli, cc-api | ADR-18; DDD-11 | bad digest, chain and missing evidence tests | Output rejected unless internally consistent |
| `cc-recovery` | Idempotent rollback planning/execution | Recovery | `RecoveryPlan::new, RecoveryExecutor::execute` | cc-orchestrator | ADR-20; DDD-12 | retry and partial-failure tests | Harmful release returns to last good digest |
| `cc-config` | Environment-derived validated runtime config | Operations | `RuntimeConfig::from_map` | cc-orchestrator, cc-api, cc-cli | ADR-21; DDD-01 | bounds/default/redaction tests | No secrets persisted; invalid config fails |
| `cc-telemetry` | Structured domain telemetry and counters | Operations | `Telemetry::emit, Telemetry::count` | cc-orchestrator, cc-evaluation | ADR-22; DDD-01 | stable event and counter tests | Every transition is observable |
| `cc-evaluation` | Scenario runner, benchmark and ablation model | Evaluation | `Evaluator::run, Variant` | cc-cli | ADR-23; DDD-04 | combined and all ablation tests | Results use frozen fixtures and seed |
| `cc-orchestrator` | End-to-end rollout saga owner | Application | `RolloutOrchestrator::run` | cc-api, cc-cli, integration tests | ADR-24; all DDDs | healthy/harmful/degraded integration tests | Crosses all mandatory boundaries |
| `cc-api` | Typed service request/response facade | Interfaces | `RolloutApi::evaluate` | external callers | ADR-25; DDD-02 | input validation and response verification | No domain policy duplicated at edge |
| `cc-cli` | Operator executable and evidence renderer | Interfaces | `canary-covenant run|evaluate|verify` | operator | ADR-25; DDD-02 | CLI success/failure snapshots | Machine-readable output and nonzero failures |

## Dependency direction

`types ← domain crates ← adapters/intelligence ← policy/recovery ← orchestrator ← api/cli`. `evaluation` invokes the orchestrator and never the reverse. No domain crate imports OpenFeature, RuVector, or RVF directly. Only `cc-openfeature`, `cc-ruvector`, and `cc-rvf` may import their respective upstream.

## Data ownership

The release aggregate owns lifecycle; the cohort plan owns ordering; the exposure ledger owns unique admissions; the e-process owns sequential wealth; the policy engine owns decisions but not execution authority; the grant aggregate owns single use; recovery owns rollback idempotency; the audit ledger owns append order; RVF is a portable projection of evidence, not the policy source of truth.

## Failure dominance

Invalid candidate > configuration error > OpenFeature unavailable/disabled > RVF verification failure > policy denial > RuVector unavailable > telemetry failure. Admission and promotion fail closed on OpenFeature/RVF/policy. RuVector loss permits hold/rollback but never promotion. Telemetry loss is surfaced and the ledger remains authoritative.

## Concurrency

Each rollout has one orchestrator writer guarded by release id. Observations may be batched but are applied in stable sequence order. Grant consumption and recovery actions use idempotency keys. Read-only verification can run concurrently against an immutable report.
