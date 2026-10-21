# Domain model index

| DDD | Bounded context / model |
| --- | --- |
| [DDD-01](DDD-01-ubiquitous-language.md) | Ubiquitous language and shared kernel |
| [DDD-02](DDD-02-release-control.md) | Release candidate aggregate |
| [DDD-03](DDD-03-cohorts-exposure.md) | Cohort and exposure context |
| [DDD-04](DDD-04-observations-evaluation.md) | Observation and evaluation context |
| [DDD-05](DDD-05-feature-eligibility.md) | Feature eligibility context |
| [DDD-06](DDD-06-incident-memory.md) | Incident memory context |
| [DDD-07](DDD-07-sequential-evidence.md) | Sequential evidence context |
| [DDD-08](DDD-08-risk-assessment.md) | Risk assessment context |
| [DDD-09](DDD-09-planning-policy.md) | Planning and policy context |
| [DDD-10](DDD-10-grant-authority.md) | Grant authority context |
| [DDD-11](DDD-11-audit-witness.md) | Audit and witness context |
| [DDD-12](DDD-12-recovery.md) | Recovery context |

## Context map

Release Control supplies admission/exposure to Evidence Intelligence. Evidence Intelligence supplies immutable assessments to Governance. Governance emits decisions and grants to the application saga. Recovery consumes rollback decisions. Operations observes every context and Evaluation is a downstream customer; neither may promote. Upstream adapters are anti-corruption layers owned by their named context.
