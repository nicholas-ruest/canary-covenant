# ADR-18: Keep verification outside orchestration

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

The writer verifying itself inline can accidentally reuse corrupted state.

## Decision

`cc-verifier` consumes an immutable report, recomputes digest bindings and calls the RVF verifier.

## Tradeoffs

It is process-independent in code ownership, not a separate trust domain.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Bad release id, missing integration, invalid decision and tampered chain are rejected.
