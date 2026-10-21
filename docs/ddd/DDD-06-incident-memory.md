# DDD-06: Incident memory context

## Responsibility and model

IncidentSet owns historical failures; IncidentMemory is a port with RuVector adapter. Embeddings are deterministic value objects. Recall returns bounded typed matches with finite scores; no result can authorize release.

## Ubiquitous language, aggregates, entities, and value objects

Incident, Severity, IncidentEmbedding, IncidentMatch.

## Invariants and domain events

Seed/search roundtrip and NoRuVector behavior prove the boundary. Domain events use past-tense names and carry release id plus monotonic sequence when applicable.

## Context relationships and ownership

Embedder feeds RuVector ACL; risk engine consumes matches. The owner is responsible for validation and state transitions; consumers receive typed snapshots or events and do not mutate internals.

## Ports and adapters

Public Rust traits/types are the ports. Upstream technology appears only in the adapter crate named in `docs/architecture.md`. Tests include domain invariants, adapter contracts when applicable, and one assertion in the rollout saga.
