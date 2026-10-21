# ADR-21: Use validated environment-derived configuration

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Committed config secrets and implicit defaults are unsafe.

## Decision

Parse an injected string map, bound alpha/min-samples/ceilings, redact sensitive keys, and expose immutable config.

## Tradeoffs

No dynamic config reload in this version.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Default, override, malformed and redaction tests execute.
