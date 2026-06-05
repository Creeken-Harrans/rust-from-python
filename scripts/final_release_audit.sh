#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
REPORT_DIR="${ROOT_DIR}/final_audit_reports"

cd "${ROOT_DIR}"
mkdir -p "${REPORT_DIR}"

echo "============================================================"
echo "Rust From Python - Final Release Audit"
echo "Root: ${ROOT_DIR}"
echo "============================================================"

echo
echo "[1/13] Toolchain"
{
  rustc --version
  cargo --version
  rustup --version || true
  python3 --version
} | tee "${REPORT_DIR}/toolchain.txt"

echo
echo "[2/13] Inventory"
python3 scripts/final_inventory.py

echo
echo "[3/13] Exercise-to-solution mapping"
python3 scripts/final_audit_solutions_mapping.py

echo
echo "[4/13] Existing solution audit"
python3 scripts/audit_solutions.py

echo
echo "[5/13] Markdown links"
python3 scripts/final_audit_markdown_links.py

echo
echo "[6/13] Formatting"
cargo fmt --all -- --check

echo
echo "[7/13] Cargo metadata"
cargo metadata --format-version 1 \
  > "${REPORT_DIR}/cargo_metadata.json"

echo
echo "[8/13] Cargo check"
cargo check --workspace --all-targets

echo
echo "[9/13] Cargo tests"
cargo test --workspace

echo
echo "[10/13] Clippy"
cargo clippy \
  --workspace \
  --all-targets \
  --all-features \
  -- \
  -D warnings

echo
echo "[11/13] Rustdoc"
RUSTDOCFLAGS="-D warnings" \
  cargo doc \
  --workspace \
  --no-deps

echo
echo "[12/13] Existing course audit"
./scripts/audit_course.sh

echo
echo "[13/13] Existing solution validation"
./scripts/check_solutions.sh

echo
echo "============================================================"
echo "Final automated release audit passed."
echo "Reports: ${REPORT_DIR}"
echo "============================================================"
