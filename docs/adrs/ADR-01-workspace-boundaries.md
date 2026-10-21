# ADR-01: Adopt 24 behavior-bearing crates aligned to bounded contexts

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

A monolith reduces build friction; a few large layer crates reduce coordination; 24 responsibility crates make ownership, testing and upstream boundaries explicit.

## Decision

Use 24 crates with typed APIs and real dependency edges. Reject function-per-crate splitting and require each member in the end-to-end call path or as a useful executable.

## Tradeoffs

More manifests and compile graph complexity, offset by enforceable ownership and targeted tests.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Workspace metadata, dependency graph, per-crate tests and end-to-end coverage must prove every member is linked.
