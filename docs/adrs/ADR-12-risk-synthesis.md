# ADR-12: Separate statistical evidence from operational risk

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

An e-value alone ignores severe historical incidents and budget exhaustion; one opaque score hides causes.

## Decision

Return a structured assessment with wealth, harm rate, incident penalty, budget remaining and reasons.

## Tradeoffs

Policy still needs calibrated thresholds; reasons prevent score laundering.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Component-level unit tests and NoRuVector delta verify attribution.
