# ADR-15: Require an attenuated one-use grant for promotion

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

A decision record alone can be replayed or applied to another cohort.

## Decision

Issue a grant bound to release, cohort, ceiling and decision hash; consuming it changes state atomically and second use fails.

## Tradeoffs

No distributed consensus is claimed; local state machine semantics are the contract.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Scope mismatch, expiry and replay tests execute.
