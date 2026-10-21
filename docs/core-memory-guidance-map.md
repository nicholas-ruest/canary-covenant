# Core-Memory guidance map

Repository guidance source commit: `f57cc3deebe89f2ec0fa2369e05ddf76a56db965`. Live retrieval status: `LIVE_MEMORY_EMPTY_UNSCREENED` — no playbook or anti-pattern entries were returned; repository and explicit user requirements remain authoritative.

| Source | Instruction | Concrete decision | Evidence target | Status |
| --- | --- | --- | --- | --- |
| `AGENTS.md` @ source commit | Canonical standard first; 20 meaningful crates; design before code | 24-crate contract written and committed before implementation | design commit, architecture crate table | APPLIED |
| `AGENTS.md` | Ruflo coordinates; Codex executes | Ruflo ledger records a bounded one-writer run; source is implemented and tested independently | executor and Ruflo ledgers | APPLIED |
| `AGENTS.md` | One writer; exact source receipt | Single integration owner on one isolated branch; evidence names indexed tree/commit | git/status receipts | APPLIED |
| `CLAUDE.md` | Validate inputs; no secrets; files focused | typed constructors, environment config, no `.env`, files below 500 lines | tests, secret scan, line-count gate | APPLIED |
| `docs/guidance/dream-machine-build-standard.md` | Real Ruvnet + enterprise + frontier composition | OpenFeature + RuVector + RVF + e-process call paths | integration tests and ablations | APPLIED |
| `docs/architecture-overview.md` | ACLs around upstream; evaluators do not self-promote | dedicated adapter crates; policy/grant owns authority | dependency review | APPLIED |
| `docs/ddd/bounded-context-map.md` | Explicit ownership, aggregates, ports and relationships | 12 DDD documents and context map | DDD index/readback | APPLIED |
| ADR example `0023-bounded-overfetch-search.md` | Decisions require alternatives, tradeoffs and validation | every project ADR contains those sections | ADR quality audit | APPLIED |
| Explicit daily override | New project, same-night target, no old-repo maintenance | Canary Covenant selected after GitHub novelty search | daily ledger and repo search | APPLIED |
| Core-memory carry-across-runs wording | Continue same system across nights | Superseded by explicit Oct 16 daily-new-build override | daily ledger | DEVIATED — higher-priority explicit instruction |
