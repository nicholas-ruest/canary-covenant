# Architecture decision index

| ADR | Decision | Status |
| --- | --- | --- |
| [ADR-01](ADR-01-workspace-boundaries.md) | Adopt 24 behavior-bearing crates aligned to bounded contexts | Accepted |
| [ADR-02](ADR-02-release-identity.md) | Bind releases to normalized name, version and 64-hex digest | Accepted |
| [ADR-03](ADR-03-validated-value-objects.md) | Use constructors for ids, percentages, risk and sequence numbers | Accepted |
| [ADR-04](ADR-04-ordered-cohorts.md) | Model rollout as an ordered cohort plan | Accepted |
| [ADR-05](ADR-05-unique-exposure-ledger.md) | Count unique admissions append-only | Accepted |
| [ADR-06](ADR-06-observation-contract.md) | Normalize harm observations before inference | Accepted |
| [ADR-07](ADR-07-openfeature-boundary.md) | Execute the official SDK behind a local provider adapter | Accepted |
| [ADR-08](ADR-08-incident-domain.md) | Represent prior rollout failures as typed incidents | Accepted |
| [ADR-09](ADR-09-deterministic-embedding.md) | Use a transparent four-feature embedding for reproducibility | Accepted |
| [ADR-10](ADR-10-ruvector-acl.md) | Confine RuVector operations to one integrity-checking adapter | Accepted |
| [ADR-11](ADR-11-anytime-valid-eprocess.md) | Use a betting e-process for continuously monitored harm | Accepted |
| [ADR-12](ADR-12-risk-synthesis.md) | Separate statistical evidence from operational risk | Accepted |
| [ADR-13](ADR-13-planner-policy-split.md) | Planner proposes while policy decides | Accepted |
| [ADR-14](ADR-14-fail-closed-dominance.md) | Define dependency-failure precedence | Accepted |
| [ADR-15](ADR-15-single-use-grants.md) | Require an attenuated one-use grant for promotion | Accepted |
| [ADR-16](ADR-16-rvf-witness-chain.md) | Use RVF crypto for portable witness binding | Accepted |
| [ADR-17](ADR-17-append-only-audit.md) | Maintain a deterministic hash-linked event ledger | Accepted |
| [ADR-18](ADR-18-independent-verifier.md) | Keep verification outside orchestration | Accepted |
| [ADR-19](ADR-19-integration-degraded-modes.md) | Ablate each upstream explicitly | Accepted |
| [ADR-20](ADR-20-idempotent-recovery.md) | Model rollback as an idempotent plan | Accepted |
| [ADR-21](ADR-21-environment-config.md) | Use validated environment-derived configuration | Accepted |
| [ADR-22](ADR-22-structured-telemetry.md) | Emit domain events independent of logs | Accepted |
| [ADR-23](ADR-23-frozen-evaluation.md) | Freeze fixtures, seed and variants | Accepted |
| [ADR-24](ADR-24-orchestrated-saga.md) | Use one application saga for rollout decisions | Accepted |
| [ADR-25](ADR-25-interfaces-no-domain-duplication.md) | Keep API and CLI as thin verified edges | Accepted |
