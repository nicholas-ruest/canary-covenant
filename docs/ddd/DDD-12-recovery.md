# DDD-12: Recovery context

## Responsibility and model

RecoveryPlan is rooted at release id and last-good digest. RecoveryExecutor applies ordered compensations with idempotency keys. The context cannot promote; successful rollback emits RecoveryCompleted for the ledger.

## Ubiquitous language, aggregates, entities, and value objects

RecoveryPlan, RecoveryAction, IdempotencyKey, RecoveryOutcome.

## Invariants and domain events

Retries are safe; partial failure is explicit; wrong release/digest rejected. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Policy rollback decision triggers plan creation; telemetry/ledger observe outcomes. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
