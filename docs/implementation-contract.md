# Implementation contract

## Stage order

1. Commit this research/design package before creating Rust workspace members.
2. Implement typed domain/value crates, then upstream adapters, then governance/recovery, then orchestration/interfaces.
3. Run the realistic healthy and harmful workloads through the official integrations.
4. Produce raw evidence, benchmark, baseline and ablation artifacts bound to the exact indexed git state.
5. Run the pinned checker and independently review substance/call paths before any repository publication.

## Completion gates

- All 24 crates are workspace members, contain nontrivial behavior and tests, and have a caller or executable entrypoint.
- `Cargo.lock` exists; `cargo fmt --check`, strict all-target/all-feature Clippy, and `cargo test --locked` pass.
- OpenFeature provider call, RuVector insert/search, and RVF create/verify execute in integration tests and the CLI workload.
- Healthy fixture promotes, harmful fixture rolls back, and grant replay/tampered witness fail.
- Benchmark includes combined, enterprise-only, Ruvnet-only, and each major ingredient removal with frozen inputs.
- Dependency/license/security gate runs (`cargo deny` where available, otherwise documented `cargo audit` + license inventory).
- Ruflo and MetaHarness/Darwin/Flywheel commands have exact version, command, exit code, source checkout and transcript.
- README includes quickstart, realistic I/O, maps, evidence table, limits, licensed marks and two script-free animated SVGs that remain legible when animation is stopped.
- Checker at `nicholas-ruest/ruvnet-dream-machine@80f006f6960bc4e491e97f51e890ff708e81aa88` exits 0 on the indexed checkout; semantic review independently confirms boundaries and evidence.
- Publication occurs on a feature branch via non-draft PR and governed merge; exact main SHA, CI, repository readback, and two owned Gists are verified.

## Explicit non-goals

No production deployment, traffic mutation, external flag service, causal treatment-effect claim, automatic key custody, or self-modifying policy. The provider is deterministic and local for reproducibility while still executing the official SDK contract.
