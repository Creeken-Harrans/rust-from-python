#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir

Set-Location $RootDir

Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "  Rust 教程全局检查" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "[INFO] Rust 工具链版本:" -ForegroundColor Yellow
rustc --version
cargo --version
Write-Host ""

Write-Host "[STEP 1/5] cargo fmt --all -- --check" -ForegroundColor Yellow
cargo fmt --all -- --check
Write-Host "[OK] Formatting check passed." -ForegroundColor Green
Write-Host ""

Write-Host "[STEP 2/5] cargo check --workspace --all-targets" -ForegroundColor Yellow
cargo check --workspace --all-targets
Write-Host "[OK] Type check passed." -ForegroundColor Green
Write-Host ""

Write-Host "[STEP 3/5] cargo test --workspace" -ForegroundColor Yellow
cargo test --workspace
Write-Host "[OK] All tests passed." -ForegroundColor Green
Write-Host ""

Write-Host "[STEP 4/5] cargo clippy --workspace --all-targets --all-features -- -D warnings" -ForegroundColor Yellow
cargo clippy --workspace --all-targets --all-features -- -D warnings
Write-Host "[OK] Clippy check passed." -ForegroundColor Green
Write-Host ""

Write-Host "[STEP 5/5] cargo doc --workspace --no-deps" -ForegroundColor Yellow
$env:RUSTDOCFLAGS = "-D warnings"
cargo doc --workspace --no-deps
Write-Host "[OK] Documentation build passed." -ForegroundColor Green
Write-Host ""

Write-Host "=========================================" -ForegroundColor Cyan
Write-Host "  All checks passed!" -ForegroundColor Cyan
Write-Host "=========================================" -ForegroundColor Cyan
