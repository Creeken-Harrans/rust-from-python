#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

cd "${ROOT_DIR}"

echo "========================================="
echo "  Rust 教程全局检查"
echo "========================================="
echo ""

echo "[INFO] Rust 工具链版本:"
rustc --version
cargo --version
echo ""

echo "[STEP 1/5] cargo fmt --all -- --check"
cargo fmt --all -- --check
echo "[OK] Formatting check passed."
echo ""

echo "[STEP 2/5] cargo check --workspace --all-targets"
cargo check --workspace --all-targets
echo "[OK] Type check passed."
echo ""

echo "[STEP 3/5] cargo test --workspace"
cargo test --workspace
echo "[OK] All tests passed."
echo ""

echo "[STEP 4/5] cargo clippy --workspace --all-targets --all-features -- -D warnings"
cargo clippy --workspace --all-targets --all-features -- -D warnings
echo "[OK] Clippy check passed."
echo ""

echo "[STEP 5/5] cargo doc --workspace --no-deps"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
echo "[OK] Documentation build passed."
echo ""

echo "========================================="
echo "  All checks passed!"
echo "========================================="
