# DDD-07: Sequential evidence context

## Responsibility and model

EProcess is the aggregate root. It owns p0, alpha, predictable bet, wealth, count and factor trace. Observation order is immutable; wealth must remain finite/nonnegative. Crossing `1/alpha` creates EvidenceReached, not Promote.

## Aggregates, entities, and value objects

EProcess, Bet, EValue, EvidenceReached.

## Invariants and domain events

Optional-stopping and null-path simulations validate the invariant. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Metrics feed observations; risk reads a snapshot; policy cannot rewrite wealth. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
