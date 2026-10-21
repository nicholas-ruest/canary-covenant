# Specification

## Original problem

Agent runtime updates combine models, tools, prompts, routing rules, and safety policy. Conventional percentage canaries answer “who receives the flag” but not whether repeated inspection has invalidated the decision, whether a recalled historical incident should lower the exposure ceiling, or whether the promotion receipt is tamper evident. Operators need one control plane that evaluates enterprise feature flags, continuously accumulates statistically valid evidence, retrieves structurally similar failures, issues single-use rollout grants, and can prove why it promoted or rolled back.

## Intended users

- Platform reliability engineers rolling out agent runtimes across tenant cohorts.
- AI safety and governance owners who define non-negotiable harm/error budgets.
- Release engineers who need deterministic promotion, rollback, and audit evidence.
- Incident responders who need prior failure context during a live canary.

## System capability

Canary Covenant turns a signed release candidate and a stream of cohort observations into a policy-bound **hold, promote, or rollback** decision, using OpenFeature targeting, RuVector incident recall, anytime-valid e-process evidence, single-use grants, RVF witness chains, and deterministic recovery.

No single ingredient provides this combined capability: OpenFeature evaluates flags but does not infer rollout safety; RuVector recalls incidents but does not authorize exposure; RVF seals evidence but does not choose a decision; an e-process supports optional stopping but has no release policy, cohorts, or recovery semantics.

## Bounded contexts

1. **Release Control** — release identity, cohort plans, OpenFeature eligibility, exposure ledger.
2. **Evidence Intelligence** — metrics normalization, RuVector incident recall, anytime-valid evidence, risk synthesis.
3. **Governance** — policy evaluation, single-use grants, RVF witness chain, audit ledger.
4. **Recovery** — rollback plan, idempotent execution, degraded-mode dominance.
5. **Operations & Evaluation** — configuration, telemetry, API/CLI, benchmarks and ablations.

## End-to-end workload

An operator proposes `agent-runtime-2.7.0`. The system validates its digest and plans `internal → beta → production`. OpenFeature evaluates whether each cohort is enabled. RuVector retrieves the nearest historical rollout incidents from fixed four-dimensional embeddings. Each observation updates a betting e-process for the unacceptable-regression null. Policy combines enterprise flag state, incident severity, e-value, exposure ceiling, and error budget. A promote decision creates a single-use grant; a rollback decision creates an idempotent recovery plan. Every material decision is appended to a hash-linked ledger and encoded in an RVF witness chain. The CLI emits a machine-readable report and verifies the chain before exit.

## Measurable success criteria

- One realistic scenario crosses all 24 crates and executes OpenFeature, RuVector, and RVF code.
- The combined system promotes the healthy fixture and rolls back the harmful fixture deterministically.
- Optional stopping does not change the configured false-promotion bound; an e-value threshold of `1/alpha` is enforced.
- Removing RuVector changes the recalled-incident risk and exposure ceiling on the seeded incident fixture.
- Removing OpenFeature denies cohort admission rather than silently allowing exposure.
- Removing the e-process prevents statistical promotion; removing RVF makes the receipt unverifiable.
- Reusing a grant or replaying a recovery action is rejected/idempotent and tested.
- Locked tests, strict Clippy, formatting, dependency policy, benchmark and all ingredient ablations produce source-bound receipts.

## Constraints

Rust core; no secrets; inputs validated; deterministic fixtures and seed; one integration writer; append-only evidence; no self-promotion by evaluators; upstream outages fail closed for admission and promotion; files remain focused; public APIs are typed and errors explicit.
