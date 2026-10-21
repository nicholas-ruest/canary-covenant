# ADR-08: Represent prior rollout failures as typed incidents

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Unstructured notes cannot drive a deterministic safety penalty.

## Decision

Incidents contain stable id, normalized embedding features, severity, affected component and mitigation.

## Tradeoffs

Manual severity calibration can bias exposure; it is visible and ablated.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Dominance and severity ordering tests use fixed incidents.
