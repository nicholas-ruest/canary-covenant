# DDD-09: Planning and policy context

## Responsibility and model

Planner proposes Hold/Promote/Rollback. PolicyEngine is a separate aggregate/service applying flags, evidence, incidents, budgets, witness readiness and degraded modes. PolicyDecision includes every gate result.

## Aggregates, entities, and value objects

Proposal, PolicyInput, PolicyDecision, DenialReason.

## Invariants and domain events

No-self-promotion is structural: planner has no grant port. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Planner is upstream and conformist; policy exposes the decision to orchestration. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
