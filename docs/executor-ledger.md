# Executor ledger

| Stage | Owner | Machine/worktree | Tool/version | Outcome |
| --- | --- | --- | --- | --- |
| Research/design | Codex integration owner | local design generation then `/home/ruv/work/canary-covenant` | Codex + GitHub connector | Design checkpoint prepared before source |
| Implementation | Codex integration owner | isolated feature branch on RuOS `8ee959f7363638` | Rust 1.99.0 / Cargo 1.99.0 | Pending |
| Validation | Codex integration owner | same immutable candidate | checker + Cargo + upstreams | Pending |

Maximum writing concurrency is one. No other worker may edit the worktree or shared manifests.
