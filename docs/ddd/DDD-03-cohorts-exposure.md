# DDD-03: Cohort and exposure context

## Responsibility and model

CohortPlan owns ordered stages and ceilings; ExposureLedger owns unique admissions. A promotion changes only one stage. Subjects cannot be admitted twice for the same release/cohort and total exposure cannot exceed the stage ceiling.

## Aggregates, entities, and value objects

CohortPlan, CohortStage, ExposureLedger, ExposureRecord.

## Invariants and domain events

Order, ceiling, uniqueness and monotonic totals are tested. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

OpenFeature supplies eligibility; policy supplies a decision; neither mutates exposure directly. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
