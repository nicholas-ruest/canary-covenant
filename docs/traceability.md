# Design-to-implementation traceability

| Requirement | Design | Planned code | Evidence |
| --- | --- | --- | --- |
| Progressive cohort admission | ADR-04, ADR-07; DDD-03/05 | `cc-cohort`, `cc-openfeature`, orchestrator | OpenFeature integration test and CLI trace |
| Anytime-valid promotion | ADR-11–14; DDD-07–09 | `cc-eprocess`, `cc-risk`, `cc-planner`, `cc-policy` | optional-stop tests and ablation |
| Historical incident influence | ADR-08–10; DDD-06/08 | `cc-incidents`, `cc-embedding`, `cc-ruvector` | real insert/search receipt and NoRuVector delta |
| Tamper-evident audit | ADR-16–18; DDD-11 | `cc-rvf`, `cc-ledger`, `cc-verifier` | RVF verify/tamper tests and sealed output |
| Bounded promotion authority | ADR-15; DDD-10 | `cc-grant` | grant replay rejection |
| Deterministic rollback | ADR-20; DDD-12 | `cc-recovery` | harmful scenario and retry test |
| Enterprise operation | ADR-21–25 | config, telemetry, evaluation, orchestrator, api, cli | strict gates, benchmark, CI and readback |
