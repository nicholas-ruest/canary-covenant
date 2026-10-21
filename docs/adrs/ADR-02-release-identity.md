# ADR-02: Bind releases to normalized name, version and 64-hex digest

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Tags are mutable; version alone cannot identify exact runtime contents; full artifact signing is outside tonight's runtime scope.

## Decision

Define `ReleaseCandidate` with validated semantic label and immutable digest, and bind all events/grants to its release id.

## Tradeoffs

Digest validation proves identity binding, not artifact provenance or signer identity.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Reject malformed digests and mismatched report digests; round-trip the exact id in CLI output.
