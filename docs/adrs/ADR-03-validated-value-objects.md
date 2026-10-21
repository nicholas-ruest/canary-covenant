# ADR-03: Use constructors for ids, percentages, risk and sequence numbers

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Primitive strings/floats invite invalid states; pervasive validation in callers drifts.

## Decision

Centralize nonempty ids, finite bounded values and monotonic sequence rules in `cc-types`.

## Tradeoffs

Small wrapper overhead and explicit conversions improve invariants.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Property/unit tests cover boundaries, NaN, overflow and empty inputs.
