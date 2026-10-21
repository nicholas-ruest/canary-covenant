# DDD-05: Feature eligibility context

## Responsibility and model

FlagGate is the anti-corruption service around the official OpenFeature client. EvaluationContext carries release, cohort and tenant. Unknown keys and provider errors are ineligible, never default-true.

## Ubiquitous language, aggregates, entities, and value objects

FlagDecision, FlagReason, EvaluationAttributes.

## Invariants and domain events

Real SDK call plus fail-closed branch must be observable. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Provider adapter implements OpenFeature; orchestrator consumes FlagGate port. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
