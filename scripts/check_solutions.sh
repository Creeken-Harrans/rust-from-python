#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

cd "${ROOT_DIR}"

echo "============================================================"
echo "Solutions Audit"
echo "============================================================"

echo ""
echo "[1/6] Audit solution documents"
python3 scripts/audit_solutions.py || echo "  Issues found (see audit_reports/solutions_report.md)"

echo ""
echo "[2/6] Format active Rust code"
cargo fmt --all -- --check || echo "  Format issues"

echo ""
echo "[3/6] Check workspace"
cargo check --workspace --all-targets || echo "  Check issues"

echo ""
echo "[4/6] Run tests"
cargo test --workspace || echo "  Test failures"

echo ""
echo "[5/6] Run Clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings || echo "  Clippy warnings"

echo ""
echo "[6/6] Build docs"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps || echo "  Doc warnings"

echo ""
echo "============================================================"
echo "Solution audit complete."
echo "============================================================"
