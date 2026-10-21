# ADR-13: Planner proposes while policy decides

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Letting an optimizer authorize its own release violates no-self-promotion.

## Decision

Planner produces a proposal from evidence; `cc-policy` independently applies conjunctive gates and degraded-mode dominance.

## Tradeoffs

Duplicate logic is avoided by typed proposal inputs; some conservative holds remain.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Tests show a promote proposal can still be denied by flag, witness or incident state.
