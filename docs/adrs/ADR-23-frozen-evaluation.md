# ADR-23: Freeze fixtures, seed and variants

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Changing input data can manufacture benchmark wins.

## Decision

Version healthy/harmful observations and incidents in the repository; evaluator records fixture hash and variant.

## Tradeoffs

Synthetic fixtures establish behavior, not production efficacy.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Re-run equality and fixture-hash checks execute.
