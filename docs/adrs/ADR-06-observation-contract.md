# ADR-06: Normalize harm observations before inference

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Raw service metrics have incompatible scales and missingness.

## Decision

Accept a typed binary harm indicator plus latency and success fields, rejecting non-finite or out-of-range values.

## Tradeoffs

Binary harm simplifies the first e-process and discards severity magnitude.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Validation and frozen-batch summaries are tested.
