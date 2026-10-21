# DDD-01: Ubiquitous language and shared kernel

## Responsibility and model

Defines Release Candidate, Cohort, Exposure, Observation, Incident, E-process, Proposal, Decision, Grant, Recovery Plan, Ledger Event and Witness Chain. IDs are opaque nonempty values; percentages and risk are finite [0,1]. Cross-context values live only in `cc-types`; behavior remains in owning contexts.

## Aggregates, entities, and value objects

ReleaseId, CohortId, Percentage, Decision, Sequence.

## Invariants and domain events

Reject primitive-string leakage at public APIs and invalid numeric states. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

All contexts consume these immutable values; changes require compatibility review. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
