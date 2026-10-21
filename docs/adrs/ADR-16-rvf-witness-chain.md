# ADR-16: Use RVF crypto for portable witness binding

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

A plain JSON report can be edited after evaluation; a custom hash chain would duplicate upstream.

## Decision

Convert material ledger events to RVF `WitnessEntry` values, call `create_witness_chain`, and verify with `verify_witness_chain`.

## Tradeoffs

A witness chain proves tamper evidence, not external timestamp notarization.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Real chain creation, verification and middle-entry tamper failure execute.
