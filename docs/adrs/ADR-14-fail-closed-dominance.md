# ADR-14: Define dependency-failure precedence

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Compound failures otherwise yield inconsistent decisions.

## Decision

Invalid input, flag failure, witness failure and policy denial dominate; RuVector loss forces hold; telemetry loss is disclosed.

## Tradeoffs

Availability is sacrificed for safe promotion semantics.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Table-driven compound-failure tests cover every pair.
