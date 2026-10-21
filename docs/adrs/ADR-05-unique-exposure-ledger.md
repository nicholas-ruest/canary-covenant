# ADR-05: Count unique admissions append-only

## Status

Accepted for the 2026-10-20 design checkpoint.

## Context and alternatives

Metric streams can duplicate delivery; mutable counters conceal replays.

## Decision

Record `(release, cohort, subject)` once and derive totals from entries.

## Tradeoffs

Memory growth is accepted for bounded fixtures; production compaction is deferred.

## Consequences

This decision is normative for Canary Covenant. Any implementation deviation must update this ADR and traceability before code changes. The owning crates and callers are listed in `docs/architecture.md`; no adapter may gain promotion authority through convenience APIs.

## Validation

Duplicate admission is idempotent and cannot inflate exposure.
