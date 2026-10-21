# ADR-04: Model rollout as an ordered cohort plan

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

A raw percentage lacks tenant semantics; arbitrary jumps defeat canary containment.

## Decision

Use named stages internal, beta, production with nondecreasing ceilings and explicit next transition.

## Tradeoffs

Fixed order is less flexible than arbitrary graphs but deterministic and auditable.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Tests reject duplicate stages, decreasing ceilings and skipped promotions.
