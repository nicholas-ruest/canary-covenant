# Research and selection

## Candidate comparison

| Candidate | System value | Composition depth | Distinctness | Decision |
| --- | --- | --- | --- | --- |
| Canary Covenant | Live agent-runtime progressive delivery with anytime-valid evidence and cryptographic receipts | OpenFeature + RuVector + RVF + e-processes | New release-control domain | Selected |
| Prompt Quarantine | Detect unsafe prompt bundle changes | Mostly validator/scorer shaped | Too close to a checker utility | Rejected |
| Agent Cost Governor | Allocate model spend across jobs | Valuable but dominated by policy/accounting | Risk of one-feature service | Rejected |

## Primary sources and pins

- OpenFeature Rust SDK `0.3.0`, commit `4ad89248135c7866ee170f384cb2da574429d9f3`, Apache-2.0: <https://github.com/open-feature/rust-sdk/tree/4ad89248135c7866ee170f384cb2da574429d9f3>. The official `FeatureProvider` interface and `OpenFeature` client are executed, not reimplemented.
- RuVector repository commit `46d816ddc4c2a8a06cd3613534a79b1d557d858b`, MIT: <https://github.com/ruvnet/RuVector/tree/46d816ddc4c2a8a06cd3613534a79b1d557d858b>. `ruvector-core` performs real insert/search on incident embeddings.
- RVF crypto `0.2.0` in the same pinned repository, MIT OR Apache-2.0. `create_witness_chain` and `verify_witness_chain` execute on release events.
- OpenFeature specification context/provider semantics: <https://openfeature.dev/specification/>.
- Ramdas, Grünwald, Vovk, and Shafer, “Game-Theoretic Statistics and Safe Anytime-Valid Inference,” arXiv:2210.01948: <https://arxiv.org/abs/2210.01948>. The system uses a nonnegative betting wealth process and Ville threshold, not a fixed-horizon p-value repeatedly peeked at.
- Ruflo runtime `3.47.1` is coordination/provenance only. It is not counted as the implementation.
- MetaHarness checkout `b8c3d38de41203faf633ef6dceb8452a4727b0ea`; Darwin `0.10.3`; Flywheel `0.1.12`. These are build/evaluation receipts and cannot promote a release.

## Frontier mechanism

For Bernoulli harm indicators `x_t ∈ {0,1}`, the null is that harm probability is at least the unacceptable boundary `p0`. A predictable negative bet `λ_t ∈ [-1/(1-p0), 0]` updates wealth `E_t = E_{t-1}(1 + λ_t(x_t-p0))`. Under the null this is a nonnegative supermartingale. Promotion requires `E_t ≥ 1/alpha`, a minimum sample count, flag eligibility, no dominant incident, and policy approval. Continuous inspection and data-dependent stopping are therefore architectural primitives rather than test-harness conveniences.

The implementation deliberately uses a conservative fixed predictable bet for reproducibility. It does not claim optimal power, causal attribution, or independence under arbitrary cohort interference.

## Novelty ledger

| Date | Prior output | Capability | Why Canary Covenant differs |
| --- | --- | --- | --- |
| 2026-10-16 | Provenance Firebreak | Supply-chain incident containment | Covenant controls prospective live rollout exposure and promotion. |
| 2026-10-17 | Continuity Atlas | Regional failover | Covenant manages cohorts and statistical release evidence, not regional recovery. |
| 2026-10-18 | Context Ferry | Context migration | Covenant does not transfer agent context. |
| 2026-10-19 | Counterfactual Shield | Pre-action tool-chain authorization | Covenant governs a multi-stage deployment after observations arrive. |
