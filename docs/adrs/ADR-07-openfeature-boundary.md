# ADR-07: Execute the official SDK behind a local provider adapter

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Hard-coded flags would not exercise enterprise flag semantics; a remote provider would make tests nondeterministic.

## Decision

Implement the official `FeatureProvider`, register it with `OpenFeature`, pass evaluation context, and call the typed client.

## Tradeoffs

The deterministic provider is not a production flag backend, but the SDK lifecycle/boundary is real.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Integration test proves provider metadata, context-sensitive result and fail-closed unknown flag.
