# ADR-11: Use a betting e-process for continuously monitored harm

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Repeated fixed-horizon tests inflate false promotion; fixed sample windows delay response.

## Decision

Use a predictable conservative bet and Ville threshold `1/alpha`, storing every factor and wealth.

## Tradeoffs

Conservative power may hold longer than an optimized mixture; validity and auditability win.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Null simulations, healthy fixture, harmful fixture and data-dependent stopping regression execute.
