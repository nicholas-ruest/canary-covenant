# ADR-09: Use a transparent four-feature embedding for reproducibility

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

External embedding APIs add secrets/cost and break reproducibility; random vectors have no semantic contract.

## Decision

Map normalized error rate, latency regression, tool-failure rate and policy-violation rate to four dimensions.

## Tradeoffs

The embedding is intentionally narrow and not a claim of semantic universality.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Dimension, range, and nearest-incident fixtures are stable.
