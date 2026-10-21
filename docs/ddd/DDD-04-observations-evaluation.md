# DDD-04: Observation and evaluation context

## Responsibility and model

ObservationBatch groups sequence-ordered binary harms with supporting latency/success metrics. Evaluator owns frozen fixtures and variant comparisons, never release authority.

## Aggregates, entities, and value objects

Observation, ObservationBatch, FixtureId, EvaluationResult.

## Invariants and domain events

No NaN, duplicate sequence or mutable fixture hash. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Metrics are upstream of e-process; evaluation calls orchestrator as a customer. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
