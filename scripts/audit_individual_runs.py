#!/usr/bin/env python3
"""Individual package runs audit: tests running each package independently."""

import os
import sys
import json
import subprocess

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPORT_DIR = os.path.join(ROOT, "audit_reports")
os.makedirs(REPORT_DIR, exist_ok=True)

TIMEOUT = 30  # seconds per package

# ── Get package list from cargo metadata ───────────────────────────────
try:
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1"],
        cwd=ROOT, capture_output=True, text=True, timeout=60
    )
    metadata = json.loads(result.stdout)
except Exception as e:
    print(f"Cannot get cargo metadata: {e}")
    sys.exit(1)

packages = [(p["name"], p) for p in metadata["packages"] if p["id"] in metadata["workspace_members"]]
packages.sort(key=lambda x: x[0])

results = []
total_pass = 0
total_fail = 0
total_skip = 0

def run_cmd(cmd, description):
    """Run a command with timeout, return (exit_code, stdout, stderr, error)."""
    try:
        r = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, timeout=TIMEOUT)
        return r.returncode, r.stdout, r.stderr, None
    except subprocess.TimeoutExpired:
        return -1, "", f"TIMEOUT ({TIMEOUT}s)", "TimeoutExpired"
    except Exception as e:
        return -1, "", str(e), type(e).__name__

for pkg_name, pkg in packages:
    targets = pkg.get("targets", [])
    binary_targets = [t for t in targets if "bin" in t.get("kind", [])]
    lib_targets = [t for t in targets if "lib" in t.get("kind", [])]
    test_targets = [t for t in targets if "test" in t.get("kind", [])]

    pkg_result = {
        "name": pkg_name,
        "binary_targets": [t["name"] for t in binary_targets],
        "lib_targets": [t["name"] for t in lib_targets],
        "runs": [],
        "passed": True,
    }

    # Step 1: cargo check
    code, stdout, stderr, err = run_cmd(
        ["cargo", "check", "-p", pkg_name, "--quiet"],
        f"cargo check -p {pkg_name}"
    )
    pkg_result["runs"].append({
        "command": f"cargo check -p {pkg_name}",
        "exit_code": code,
        "passed": code == 0,
        "summary": stderr[-300:] if code != 0 else "",
    })
    if code != 0:
        pkg_result["passed"] = False

    # Step 2: cargo test (if library or test targets)
    if lib_targets or test_targets:
        code, stdout, stderr, err = run_cmd(
            ["cargo", "test", "-p", pkg_name, "--quiet"],
            f"cargo test -p {pkg_name}"
        )
        pkg_result["runs"].append({
            "command": f"cargo test -p {pkg_name}",
            "exit_code": code,
            "passed": code == 0,
            "summary": stderr[-500:] if code != 0 else "",
        })
        if code != 0:
            pkg_result["passed"] = False

    # Step 3: cargo run (if binary, and not interactive)
    if binary_targets:
        for bt in binary_targets:
            bin_name = bt["name"]
            # Check if interactive
            manifest_dir = os.path.dirname(pkg["manifest_path"])
            readme_path = os.path.join(manifest_dir, "README.md")
            is_interactive = False
            try:
                with open(readme_path, "r", encoding="utf-8") as fh:
                    rmd = fh.read().lower()
            except Exception:
                rmd = ""
            if any(kw in rmd for kw in ["交互", "stdin", "用户输入", "user input", "read line", "read_line", "guess", "猜测", "输入"]):
                is_interactive = True

            if is_interactive:
                pkg_result["runs"].append({
                    "command": f"cargo run -p {pkg_name} --bin {bin_name}",
                    "exit_code": None,
                    "passed": True,
                    "summary": "Skipped (interactive program)",
                    "skipped": True,
                })
                total_skip += 1
            else:
                code, stdout, stderr, err = run_cmd(
                    ["cargo", "run", "-p", pkg_name, "--quiet"],
                    f"cargo run -p {pkg_name}"
                )
                pkg_result["runs"].append({
                    "command": f"cargo run -p {pkg_name}",
                    "exit_code": code,
                    "passed": code == 0,
                    "stdout_sample": stdout[-500:] if stdout else "",
                    "summary": stderr[-300:] if code != 0 else "",
                })
                if code != 0:
                    pkg_result["passed"] = False

    if pkg_result["passed"]:
        total_pass += 1
    else:
        total_fail += 1
    results.append(pkg_result)

# ── Report ─────────────────────────────────────────────────────────────
report_path = os.path.join(REPORT_DIR, "individual_runs_report.md")
with open(report_path, "w", encoding="utf-8") as fh:
    fh.write("# Individual Package Runs Report\n\n")
    fh.write(f"**Packages checked**: {len(packages)}\n\n")
    fh.write(f"| Status | Count |\n|--------|------|\n")
    fh.write(f"| ✅ Passed | {total_pass} |\n")
    fh.write(f"| ❌ Failed | {total_fail} |\n")
    fh.write(f"| ⏭️ Skipped (interactive) | {total_skip} |\n\n")

    fh.write("## Results Table\n\n")
    fh.write("| Package | Check | Test | Run | Overall |\n")
    fh.write("|---------|-------|------|-----|--------|\n")
    for r in results:
        check_s = ""
        test_s = ""
        run_s = ""
        for run_item in r["runs"]:
            if "check" in run_item["command"]:
                check_s = "✅" if run_item["passed"] else "❌"
            elif "test" in run_item["command"]:
                test_s = "✅" if run_item["passed"] else "❌"
            elif "run" in run_item["command"]:
                if run_item.get("skipped"):
                    run_s = "⏭️"
                else:
                    run_s = "✅" if run_item["passed"] else "❌"
        overall = "✅" if r["passed"] else "❌"
        fh.write(f"| {r['name']} | {check_s or 'N/A'} | {test_s or 'N/A'} | {run_s or 'N/A'} | {overall} |\n")

    fh.write("\n## Failures\n\n")
    for r in results:
        if not r["passed"]:
            fh.write(f"### ❌ {r['name']}\n\n")
            for run_item in r["runs"]:
                if not run_item["passed"]:
                    fh.write(f"- **{run_item['command']}** (exit={run_item['exit_code']})\n")
                    if run_item.get("summary"):
                        fh.write(f"  ```\n{run_item['summary']}\n  ```\n")
            fh.write("\n")

    if total_fail > 0:
        fh.write(f"**FAIL**: {total_fail} packages failed.\n")
    else:
        fh.write(f"**PASS**: All {total_pass} packages pass.\n")

print(f"Individual runs report: {report_path}")
print(f"Pass={total_pass}, Fail={total_fail}, Skip={total_skip}")

sys.exit(1 if total_fail > 0 else 0)
