# Validation contract

Validation binds claims to the exact indexed candidate and runs in this order: formatting, strict lint, locked workspace tests, end-to-end healthy and harmful workloads, dependency/license/advisory policy, release-mode benchmarks and ingredient ablations, upstream execution receipts, the pinned structural checker, independent semantic review, hosted CI, and final repository readback.

## Required commands

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets --all-features --locked`
- `bash tests/e2e.sh`
- `cargo deny check`
- release CLI over combined, Ruvnet-only, enterprise-only, and every major-ingredient ablation
- checker regression tests and `check-project-contract.mjs` at commit `80f006f6960bc4e491e97f51e890ff708e81aa88`

## Acceptance criteria

The healthy combined workload must promote, consume exactly one grant, recall incidents, and verify its RVF witness. The harmful combined workload must roll back, execute both recovery actions, and verify its witness. Every major-ingredient ablation must lose a capability or refuse promotion. No crate may be orphaned except executable roots. The structural checker must exit zero, after which a human-readable semantic review must still confirm real call paths and source-bound evidence.

## Evidence policy

Commands, versions, exit codes, frozen fixture hashes, raw outputs, and limitations live under `docs/evidence/`. Failed attempts remain available as diagnostic history but are never presented as passing evidence. Publication is allowed only after local gates, checker, semantic review, governed PR CI, and exact-main readback succeed.
