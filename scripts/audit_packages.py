#!/usr/bin/env python3
"""Package audit: checks Cargo workspace configuration."""

import os
import sys
import json
import subprocess
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPORT_DIR = os.path.join(ROOT, "audit_reports")
os.makedirs(REPORT_DIR, exist_ok=True)

problems_p0 = []
problems_p1 = []
problems_p2 = []

def record(level, msg, detail=""):
    d = {"level": level, "message": msg, "detail": detail}
    if level == "P0":
        problems_p0.append(d)
    elif level == "P1":
        problems_p1.append(d)
    else:
        problems_p2.append(d)

# ── Run cargo metadata ─────────────────────────────────────────────────
print("Running cargo metadata...")
try:
    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1"],
        cwd=ROOT, capture_output=True, text=True, timeout=120
    )
    if result.returncode != 0:
        record("P0", "cargo metadata failed", result.stderr[:500])
        metadata = None
    else:
        metadata = json.loads(result.stdout)
        meta_path = os.path.join(REPORT_DIR, "cargo_metadata.json")
        with open(meta_path, "w") as fh:
            json.dump(metadata, fh, indent=2)
        print(f"  Metadata saved to {meta_path}")
except Exception as e:
    record("P0", f"cargo metadata exception: {e}")
    metadata = None

if metadata is None:
    print("Cannot continue package audit without cargo metadata")
    report_path = os.path.join(REPORT_DIR, "packages_report.md")
    with open(report_path, "w") as fh:
        fh.write("# Package Audit Report\n\n**FAIL**: Cannot obtain cargo metadata.\n")
    sys.exit(1)

# ── Check root workspace ───────────────────────────────────────────────
workspace_members = metadata.get("workspace_members", [])
packages = metadata.get("packages", [])
package_by_id = {p["id"]: p for p in packages}
member_packages = [p for p in packages if p["id"] in workspace_members]

# 1. Root is virtual workspace
root_pkg = [p for p in packages if os.path.normpath(p.get("manifest_path", "")) == os.path.normpath(os.path.join(ROOT, "Cargo.toml"))]
if root_pkg:
    targets = root_pkg[0].get("targets", [])
    if targets:
        record("P1", "Root Cargo.toml should be a virtual workspace (no targets)", str(targets[:2]))
else:
    record("P0", "Root Cargo.toml not found in workspace members (may be virtual-only)")

# 2. Workspace section
root_toml = os.path.join(ROOT, "Cargo.toml")
with open(root_toml, "r") as fh:
    root_toml_content = fh.read()
if "[workspace]" not in root_toml_content:
    record("P0", "Root Cargo.toml missing [workspace] section")

# 3. Resolver
if 'resolver = "3"' not in root_toml_content and "resolver = '3'" not in root_toml_content:
    record("P0", "Workspace should use resolver = \"3\" for edition 2024")

# 4. Check expected chapters are workspace members
EXPECTED_CHAPTERS = [f"chapters/{ch}" for ch in [
    "00_course_orientation", "01_hello_cargo", "02_variables_and_types",
    "03_functions_expressions_control_flow", "04_stack_heap_and_raii",
    "05_ownership_move_copy_clone", "06_references_borrowing_slices",
    "07_ownership_practice_text_analyzer", "08_structs_methods_associated_functions",
    "09_enums_option_pattern_matching", "10_collections_vec_string_hashmap",
    "11_patterns_and_destructuring", "12_error_handling_result_question_mark",
    "13_packages_crates_modules_visibility", "14_testing_documentation_benchmindset",
    "15_generics_traits_trait_bounds", "16_lifetimes",
    "17_trait_objects_dynamic_dispatch", "18_closures_iterators",
    "19_smart_pointers_box_rc_refcell", "20_resource_management_drop_deref",
    "21_threads_channels_shared_state", "22_async_await_tokio_intro",
    "23_macros", "24_unsafe_rust_and_ffi_overview",
    "25_cargo_dependencies_features_profiles", "26_workspace_architecture",
    "27_lints_format_docs_ci",
]]

EXPECTED_PROJECTS = [
    "projects/01_guessing_game", "projects/02_cli_text_search",
    "projects/03_todo_cli", "projects/04_parallel_text_stats",
    "projects/05_mini_kv_store",
]

all_expected = EXPECTED_CHAPTERS + EXPECTED_PROJECTS

member_paths = set()
for pkg in member_packages:
    manifest = pkg.get("manifest_path", "")
    rel = os.path.relpath(os.path.dirname(manifest), ROOT)
    member_paths.add(rel)

for expected in all_expected:
    if expected not in member_paths:
        record("P0", f"Expected package not in workspace: {expected}")

# Check for extra packages
for mpath in sorted(member_paths):
    if not mpath.startswith(("chapters/", "projects/")):
        if mpath != ".":
            record("P1", f"Unexpected workspace member: {mpath}")

# 5. Package name uniqueness
names = {}
for pkg in member_packages:
    name = pkg.get("name", "")
    if name in names:
        record("P0", f"Duplicate package name: {name} in {names[name]} and {os.path.dirname(pkg['manifest_path'])}")
    names[name] = os.path.dirname(pkg.get("manifest_path", ""))

# 6. Edition check
for pkg in member_packages:
    pkg_name = pkg.get("name", "unknown")
    edition = pkg.get("edition", "")
    if edition != "2024":
        record("P1", f"Package '{pkg_name}' uses edition '{edition}' instead of '2024'")

# 7. Path dependencies
for pkg in member_packages:
    for dep in pkg.get("dependencies", []):
        dp = dep.get("path")
        if dp:
            dep_dir = os.path.normpath(os.path.join(os.path.dirname(pkg["manifest_path"]), dp))
            if not os.path.isdir(dep_dir):
                record("P0", f"Package '{pkg['name']}' has path dependency to missing directory: {dp}")

# 8. Check broken_examples is NOT a member
for mpath in member_paths:
    if "broken_examples" in mpath:
        record("P0", f"broken_examples should not be a workspace member: {mpath}")

# 9. No nested workspaces
for pkg in member_packages:
    manifest_dir = os.path.dirname(pkg["manifest_path"])
    if manifest_dir != ROOT:
        if "[workspace]" in open(pkg["manifest_path"], "r").read():
            record("P1", f"Nested workspace found in {pkg['name']}")

# 10. Cargo.lock existence
lock_path = os.path.join(ROOT, "Cargo.lock")
if not os.path.exists(lock_path):
    record("P1", "Cargo.lock is missing")

# ── Report ─────────────────────────────────────────────────────────────
report_path = os.path.join(REPORT_DIR, "packages_report.md")
total_p0 = len(problems_p0)
total_p1 = len(problems_p1)
total_p2 = len(problems_p2)

with open(report_path, "w", encoding="utf-8") as fh:
    fh.write("# Package Audit Report\n\n")
    fh.write(f"**Package count**: {len(member_packages)} workspace members\n")
    fh.write(f"**Member paths in workspace**: {len(member_paths)}\n\n")

    fh.write(f"## P0 Blocking Issues: {total_p0}\n\n")
    for p in problems_p0:
        fh.write(f"- **[P0]** {p['message']} {p.get('detail', '')}\n")

    fh.write(f"\n## P1 Quality Issues: {total_p1}\n\n")
    for p in problems_p1:
        fh.write(f"- **[P1]** {p['message']} {p.get('detail', '')}\n")

    fh.write(f"\n## P2 Optimization Issues: {total_p2}\n\n")
    for p in problems_p2:
        fh.write(f"- **[P2]** {p['message']} {p.get('detail', '')}\n")

    fh.write(f"\n## Workspace Members\n\n")
    for mpath in sorted(member_paths):
        fh.write(f"- `{mpath}`\n")

    fh.write(f"\n## Summary\n\n")
    fh.write(f"| Level | Count |\n")
    fh.write(f"|-------|-------|\n")
    fh.write(f"| P0    | {total_p0} |\n")
    fh.write(f"| P1    | {total_p1} |\n")
    fh.write(f"| P2    | {total_p2} |\n")

    if total_p0 > 0:
        fh.write(f"\n**FAIL**: {total_p0} blocking issues.\n")
    else:
        fh.write(f"\n**PASS**: No blocking issues.\n")

print(f"Package audit report written to {report_path}")
print(f"P0={total_p0}, P1={total_p1}, P2={total_p2}")

sys.exit(1 if total_p0 > 0 else 0)
