# ADR-17: Maintain a deterministic hash-linked event ledger

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Mutable latest-state logs erase intermediate holds and degraded states.

## Decision

Append canonical events with previous digest and monotonic sequence; project current state by replay.

## Tradeoffs

Storage grows and canonicalization is a compatibility surface.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Mutation and reordering change the derived digest and fail verification.
