# DDD-08: Risk assessment context

## Responsibility and model

RiskEngine creates an explainable assessment from harm rate, e-value, exposure, recalled incidents and budget. It returns components and reasons; it does not collapse missing intelligence into zero risk.

## Ubiquitous language, aggregates, entities, and value objects

RiskAssessment, RiskReason, ErrorBudget.

## Invariants and domain events

Ablation changes explicit fields and prevents false equivalence. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Customers are planner and policy; suppliers are evidence and incident memory. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
