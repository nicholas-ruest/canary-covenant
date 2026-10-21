# ADR-19: Ablate each upstream explicitly

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Fallbacks can silently turn required integrations into name-only dependencies.

## Decision

Model `Combined`, `NoOpenFeature`, `NoRuVector`, `NoRvf`, `NoEProcess`, `EnterpriseOnly`, and `RuvnetOnly` variants.

## Tradeoffs

Variants are evaluation-only and cannot be selected by the production API.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Every variant emits a distinct result and the combined path is superior on the declared scorecard.
