#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
REPORT_DIR="${ROOT_DIR}/audit_reports"

cd "${ROOT_DIR}"
mkdir -p "${REPORT_DIR}"

echo "============================================================"
echo "Rust Tutorial Full Audit"
echo "Root: ${ROOT_DIR}"
echo "============================================================"

echo ""
echo "[1/12] Toolchain versions"
rustc --version 2>&1 | tee "${REPORT_DIR}/toolchain.txt"
cargo --version 2>&1 | tee -a "${REPORT_DIR}/toolchain.txt"
rustup --version 2>&1 | tee -a "${REPORT_DIR}/toolchain.txt" || true
python3 --version 2>&1 | tee -a "${REPORT_DIR}/toolchain.txt"

echo ""
echo "[2/12] Structure audit"
python3 scripts/audit_structure.py || echo "  ⚠ Structure audit found issues"

echo ""
echo "[3/12] Markdown link audit"
python3 scripts/audit_markdown_links.py || echo "  ⚠ Link audit found issues"

echo ""
echo "[4/12] Cargo package audit"
python3 scripts/audit_packages.py || echo "  ⚠ Package audit found issues"

echo ""
echo "[5/12] Content static scan"
python3 scripts/audit_content_quality.py || echo "  ⚠ Content scan found issues"

echo ""
echo "[6/12] Rust formatting"
cargo fmt --all -- --check || echo "  ⚠ Formatting issues"

echo ""
echo "[7/12] Cargo check"
cargo check --workspace --all-targets || echo "  ⚠ Type check issues"

echo ""
echo "[8/12] Cargo tests"
cargo test --workspace || echo "  ⚠ Test failures"

echo ""
echo "[9/12] Clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings || echo "  ⚠ Clippy warnings"

echo ""
echo "[10/12] Rustdoc"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps || echo "  ⚠ Doc warnings"

echo ""
echo "[11/12] Cargo metadata"
cargo metadata --format-version 1 > "${REPORT_DIR}/cargo_metadata.json" 2>&1 || echo "  ⚠ Metadata failed"

echo ""
echo "[12/12] Independent package runs"
python3 scripts/audit_individual_runs.py || echo "  ⚠ Individual runs had failures"

echo ""
echo "============================================================"
echo "All automated audit stages completed."
echo "Reports: ${REPORT_DIR}"
echo "============================================================"
