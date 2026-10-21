# ADR-10: Confine RuVector operations to one integrity-checking adapter

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Direct imports leak storage semantics and documented API drift into domain code.

## Decision

Only `cc-ruvector` imports `ruvector-core`; it owns storage paths, ids, bounded k, finite scores and cleanup.

## Tradeoffs

Adapter code must track upstream API changes.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Real temp-store insert/search and malformed-result checks execute.
