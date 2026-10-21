# Semantic and evidence review

## System-scale capability

Canary Covenant governs an autonomous-agent runtime rollout from release intake through cohort exposure, official feature-flag resolution, historical incident recall, anytime-valid monitoring, policy, one-use authority, tamper-evident evidence, and recovery. It is not primarily a checker, score, wrapper, or single-purpose daemon.

## Meaningful crate review

All 24 crates expose a typed public API and are either called by another workspace crate or are executable roots. `cc-orchestrator` composes the domain, OpenFeature, RuVector, RVF, sequential evidence, policy, grant, ledger, recovery, configuration, and telemetry boundaries. `cc-api` and `cc-cli` are the two application roots. Tests execute each boundary rather than merely constructing it.

## Executed integrations

- OpenFeature 0.3.0: a real provider is installed in the official SDK and typed boolean resolution is called with evaluation context.
- RuVector Core 2.4.0: real `VectorDB` creation, insertion, and nearest-neighbour search run in the end-to-end path.
- RVF Crypto 0.2.0: the action ledger is bound into a SHAKE-256 witness chain and independently verified.
- Ruflo 3.47.1: bounded hierarchical coordination initialized with strict permissions and one writer.
- MetaHarness Darwin 0.10.3: seeded Darwin Shield benchmark passed its frozen acceptance gates.
- MetaHarness Flywheel 0.1.12: three deterministic generations produced signed lineage; replay with the pinned gate fingerprint passed.

## Frontier mechanism

The betting e-process updates after every observation and is compared with Ville's `1/alpha` boundary. Removing it forces Hold even on the healthy fixture; harmful observations reduce wealth and drive rollback through the risk/policy path. This is architecture-changing sequential evidence, not a citation-only claim.

## Ablation findings

Only the combined system promotes. Removing OpenFeature, RuVector, or the e-process forces Hold; removing RVF also makes the report unverifiable. Enterprise-only and Ruvnet-only baselines both hold. Five release-mode repetitions preserve decisions and witness/grant outcomes.

## Limitations accepted for this release

RuVector is in-process because 2.4.0 does not publicly export persistent `DbOptions`. The fixture uses four transparent incident features rather than a universal embedding model. Grants are single-process state machines. The OpenFeature provider is local. The `bincode 2.0.1` maintenance advisory is a transitive RuVector limitation with no safe upgrade.
