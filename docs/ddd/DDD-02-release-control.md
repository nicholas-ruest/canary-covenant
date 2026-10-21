# DDD-02: Release candidate aggregate

## Responsibility and model

The aggregate root is ReleaseCandidate. It owns candidate identity and lifecycle Draft→Validated→Active→Promoted/RolledBack. Digest mismatch, activation before validation and terminal-state mutation are forbidden.

## Aggregates, entities, and value objects

ReleaseCandidate; ReleaseValidated, ReleaseActivated, ReleasePromoted, ReleaseRolledBack.

## Invariants and domain events

Every terminal event binds the same release id and digest. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Inbound port accepts a candidate request; orchestrator calls domain methods; API/CLI are adapters. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
