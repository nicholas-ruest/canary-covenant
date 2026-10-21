# Evidence index

All receipts are source-bound to the candidate recorded by the final checker and publication readback.

| Gate | Command / boundary | Result | Raw receipt |
| --- | --- | --- | --- |
| Format | `cargo fmt --all --check` | exit 0 | `raw/cargo-fmt.txt` |
| Strict lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0 | `raw/cargo-clippy.txt` |
| Locked tests | `cargo test --workspace --all-targets --all-features --locked` | exit 0 | `raw/cargo-test-final.txt` |
| End to end | `bash tests/e2e.sh` | exit 0 | `raw/e2e.txt` |
| Dependency policy | `cargo deny check` | exit 0; explicit transitive bincode exception | `raw/cargo-deny-final.txt` |
| Healthy workload | release CLI, combined | Promote; verified; one-use grant consumed | `raw/e2e-healthy.json` |
| Harmful workload | release CLI, combined | Rollback; 2 recovery actions; verified | `raw/e2e-harmful.json` |
| Benchmark/ablation | 7 variants × 5 fixed-seed runs | only combined promotes | `raw/benchmark.csv` |
| Ruflo | bounded hierarchical swarm init | exit 0, one writer | `raw/ruflo-swarm-init.txt` |
| Darwin | Shield bench, population 16, 50 cycles, seed 0 | PASS | `raw/darwin-security-bench-final.txt` |
| Flywheel | published 0.1.12, 3 generations + replay | replay PASS; milestone reached | `raw/flywheel-live.txt` |

`RUSTSEC-2025-0141` marks transitive `bincode 2.0.1` unmaintained; it does not report a vulnerability and offers no safe upgrade. The exception is scoped in `deny.toml` and remains a documented release limitation.
