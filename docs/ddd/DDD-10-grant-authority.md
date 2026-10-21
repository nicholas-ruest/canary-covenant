# DDD-10: Grant authority context

## Responsibility and model

GrantIssuer creates RolloutGrant only from an approved decision hash. Grant binds release, cohort and ceiling and transitions Issued→Consumed exactly once. Scope mismatch, expiry and replay fail.

## Ubiquitous language, aggregates, entities, and value objects

RolloutGrant, GrantScope, GrantState, GrantConsumed.

## Invariants and domain events

All invalid transitions are unrepresentable or error-returning. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Orchestrator requests/consumes; interface layers can only display grant metadata. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
