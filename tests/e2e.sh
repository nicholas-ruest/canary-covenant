#!/usr/bin/env bash
set -euo pipefail
cargo run --locked -p cc-cli -- evaluate --variant combined > docs/evidence/raw/e2e-healthy.json
cargo run --locked -p cc-cli -- evaluate --variant combined --harmful > docs/evidence/raw/e2e-harmful.json
grep -q '"decision": "Promote"' docs/evidence/raw/e2e-healthy.json
grep -q '"decision": "Rollback"' docs/evidence/raw/e2e-harmful.json
