# ADR-22: Emit domain events independent of logs

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Free-text logs are hard to correlate and test.

## Decision

Store typed event name, release, sequence and fields; counters derive from events.

## Tradeoffs

In-memory sink is a reference adapter, not durable observability storage.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Stable names and expected event counts are asserted end to end.
