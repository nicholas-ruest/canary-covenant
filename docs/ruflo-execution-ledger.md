# Ruflo execution ledger

Ruflo is coordination/provenance, not implementation. Runtime discovered: `ruflo v3.47.1`. Planned topology: hierarchical, maximum one active writing worker in this single-worktree run. Input: the design contract. Desired output: a validated candidate commit. Acceptance: checker plus semantic/integration evidence review. Side effects: isolated files, git commits, governed repository/PR/Gists only after gates. Runtime budget: current scheduled run.

Exact coordination commands, exit codes and transcript paths are appended during execution.
