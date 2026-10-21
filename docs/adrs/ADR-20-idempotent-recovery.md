# ADR-20: Model rollback as an idempotent plan

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Retries after partial failure can repeat destructive actions.

## Decision

Bind recovery actions to release and last-good digest; executor tracks action keys and returns already-applied on replay.

## Tradeoffs

This demonstrates control-plane semantics without deploying real workloads.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Harmful scenario rolls back; duplicate execute is safe; wrong digest fails.
