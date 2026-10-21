# DDD-11: Audit and witness context

## Responsibility and model

AuditLedger is the source event sequence; WitnessChain is a portable RVF projection. Verifier is a customer of both. Reordering, mutation, missing integrations or a broken witness link invalidates the report.

## Ubiquitous language, aggregates, entities, and value objects

LedgerEvent, EventDigest, WitnessArtifact, VerificationReport.

## Invariants and domain events

Create/verify/tamper tests execute upstream code and canonicalization. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Ledger feeds RVF adapter; verifier reads immutable bytes and report fields. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
