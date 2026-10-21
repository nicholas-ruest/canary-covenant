# Ruflo execution ledger

Ruflo is coordination/provenance, not implementation. Runtime discovered: `ruflo v3.47.1`. Planned topology: hierarchical, maximum one active writing worker in this single-worktree run. Input: the design contract. Desired output: a validated candidate commit. Acceptance: checker plus semantic/integration evidence review. Side effects: isolated files, git commits, governed repository/PR/Gists only after gates. Runtime budget: current scheduled run.

Executed `ruflo swarm init -t hierarchical -m 1 -s specialized --with-permissions strict`; exit 0, swarm `swarm-1792546487082-b18zhh`. The generated permission manifest preserves a single integration writer. `ruflo memory search` returned exit 1 because this workspace had no initialized memory database; it was recorded as unavailable rather than claimed as retrieved. Raw receipts are under `docs/evidence/raw/`.
