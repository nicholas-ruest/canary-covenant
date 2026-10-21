# ADR-24: Use one application saga for rollout decisions

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Distributed callbacks obscure ordering and failure compensation.

## Decision

`cc-orchestrator` calls every domain/adapter boundary in explicit order and returns a complete report.

## Tradeoffs

Single-process throughput is sufficient for the evaluation workload.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Integration tests inspect call-path effects: exposure, recall, wealth, policy, grant/recovery, ledger and witness.
