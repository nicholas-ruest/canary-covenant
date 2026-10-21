# ADR-25: Keep API and CLI as thin verified edges

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Reimplementing policy in presentation layers causes divergence.

## Decision

API validates requests then delegates; CLI serializes verified reports and maps errors to nonzero exits.

## Tradeoffs

No network server in the initial deliverable; the typed facade is embeddable.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

API invalid-input tests and CLI JSON snapshots execute.
