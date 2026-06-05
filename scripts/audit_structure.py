#!/usr/bin/env python3
"""Structure audit: checks files exist, are non-empty, and have expected content."""

import os
import sys
import json

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPORT_DIR = os.path.join(ROOT, "audit_reports")
os.makedirs(REPORT_DIR, exist_ok=True)

problems_p0 = []
problems_p1 = []
problems_p2 = []
warnings = []

def record(level, msg, file=None, line=None):
    loc = f" ({file}:{line})" if file else ""
    entry = {"level": level, "message": msg, "file": file, "line": line}
    if level == "P0":
        problems_p0.append(entry)
    elif level == "P1":
        problems_p1.append(entry)
    else:
        problems_p2.append(entry)

# ── 5.1 Required root files ──────────────────────────────────────────
REQUIRED_ROOT = [
    "Cargo.toml", "rust-toolchain.toml", "README.md", "COURSE_MAP.md",
    "LEARNING_GUIDE.md", "PROJECT_STRUCTURE.md", "PYTHON_TO_RUST.md",
    "C_CPP_TO_RUST.md", "MENTAL_MODELS.md", "MISCONCEPTIONS.md",
    "GLOSSARY.md", "COMMANDS.md", "TROUBLESHOOTING.md",
    "PROGRESS.md", "VALIDATION.md",
    "scripts/check_all.sh", "scripts/check_all.ps1",
]

for f in REQUIRED_ROOT:
    path = os.path.join(ROOT, f)
    if not os.path.exists(path):
        record("P0", f"Missing required root file: {f}", file=f)
    elif os.path.getsize(path) == 0:
        record("P0", f"Required root file is empty: {f}", file=f)

# ── 5.2 Required chapters ─────────────────────────────────────────────
REQUIRED_CHAPTERS = [
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
]

for ch in REQUIRED_CHAPTERS:
    ch_dir = os.path.join(ROOT, "chapters", ch)
    if not os.path.isdir(ch_dir):
        record("P0", f"Missing chapter directory: chapters/{ch}")
        continue

    # Check minimum files
    for req in ["Cargo.toml", "README.md", "EXERCISES.md", "src"]:
        path = os.path.join(ch_dir, req)
        if not os.path.exists(path):
            record("P0", f"Missing {req} in chapters/{ch}")

    # Check for at least main.rs or lib.rs
    src_dir = os.path.join(ch_dir, "src")
    has_main = os.path.exists(os.path.join(src_dir, "main.rs"))
    has_lib = os.path.exists(os.path.join(src_dir, "lib.rs"))
    if not has_main and not has_lib:
        record("P0", f"No main.rs or lib.rs in chapters/{ch}/src")

# ── 5.3 Required projects ─────────────────────────────────────────────
REQUIRED_PROJECTS = [
    "01_guessing_game", "02_cli_text_search", "03_todo_cli",
    "04_parallel_text_stats", "05_mini_kv_store",
]

for prj in REQUIRED_PROJECTS:
    prj_dir = os.path.join(ROOT, "projects", prj)
    if not os.path.isdir(prj_dir):
        record("P0", f"Missing project directory: projects/{prj}")
        continue

    for req in ["Cargo.toml", "README.md", "src"]:
        path = os.path.join(prj_dir, req)
        if not os.path.exists(path):
            record("P0", f"Missing {req} in projects/{prj}")

    src_dir = os.path.join(prj_dir, "src")
    has_main = os.path.exists(os.path.join(src_dir, "main.rs"))
    has_lib = os.path.exists(os.path.join(src_dir, "lib.rs"))
    if not has_main and not has_lib:
        record("P0", f"No main.rs or lib.rs in projects/{prj}/src")

# ── 5.5 Non-empty file check ──────────────────────────────────────────
for root, dirs, files in os.walk(os.path.join(ROOT, "chapters")):
    dirs[:] = [d for d in dirs if d not in ("target", ".git")]
    for f in files:
        if f in ("main.rs", "lib.rs", "README.md", "EXERCISES.md", "Cargo.toml"):
            path = os.path.join(root, f)
            if os.path.getsize(path) == 0:
                record("P0", f"Empty file: {os.path.relpath(path, ROOT)}")

for root, dirs, files in os.walk(os.path.join(ROOT, "projects")):
    dirs[:] = [d for d in dirs if d not in ("target", ".git")]
    for f in files:
        if f in ("main.rs", "lib.rs", "README.md", "Cargo.toml"):
            path = os.path.join(root, f)
            if os.path.getsize(path) == 0:
                record("P0", f"Empty file: {os.path.relpath(path, ROOT)}")

# ── 5.6 README minimum structure check ────────────────────────────────
README_SECTIONS = [
    "目标", "背景", "核心", "运行", "代码", "讲解", "练习", "小结",
    "下一章", "对比", "对照", "迁移", "设计", "动机", "原因",
]

for ch in REQUIRED_CHAPTERS:
    readme = os.path.join(ROOT, "chapters", ch, "README.md")
    if not os.path.exists(readme) or os.path.getsize(readme) == 0:
        continue
    with open(readme, "r", encoding="utf-8") as fh:
        content = fh.read()

    found = sum(1 for kw in README_SECTIONS if kw in content)
    if found < 5:
        record("P1", f"chapters/{ch}/README.md may lack structure (only {found}/15 keyword matches)", file=f"chapters/{ch}/README.md")

# ── 5.7 Shell check ───────────────────────────────────────────────────
PLACEHOLDER_PATTERNS = ["TODO", "TBD", "PLACEHOLDER", "待补充", "稍后完成", "略", "未实现"]
for root, dirs, files in os.walk(ROOT):
    dirs[:] = [d for d in dirs if d not in ("target", ".git", "audit_reports")]
    for f in files:
        if not f.endswith((".md", ".rs")):
            continue
        # Skip EXERCISES.md (learners may have TODOs there)
        if f == "EXERCISES.md":
            continue
        path = os.path.join(root, f)
        try:
            with open(path, "r", encoding="utf-8") as fh:
                for lineno, line in enumerate(fh, 1):
                    for pat in PLACEHOLDER_PATTERNS:
                        if pat in line:
                            # Exclude legitimate uses
                            if pat == "TODO" and "cargo" in line.lower():
                                continue  # // TODO comments in teaching code
                            if pat == "略" and ("忽" in line or "省" in line or "概" in line):
                                continue
                            if pat == "未实现" and "unimplemented" in line.lower():
                                continue
                            rel_path = os.path.relpath(path, ROOT)
                            record("P1", f"Placeholder '{pat}' in {rel_path}:{lineno}: {line.strip()}")
        except Exception:
            pass

# ── Generate report ────────────────────────────────────────────────────
report_path = os.path.join(REPORT_DIR, "structure_report.md")
total_p0 = len(problems_p0)
total_p1 = len(problems_p1)
total_p2 = len(problems_p2)

with open(report_path, "w", encoding="utf-8") as fh:
    fh.write("# Structure Audit Report\n\n")
    fh.write(f"**Total files checked**: {len(REQUIRED_ROOT)} root + {len(REQUIRED_CHAPTERS)} chapters + {len(REQUIRED_PROJECTS)} projects\n\n")

    fh.write(f"## P0 Blocking Issues: {total_p0}\n\n")
    for p in problems_p0:
        loc = f" (`{p['file']}`)" if p['file'] else ""
        fh.write(f"- **[P0]** {p['message']}{loc}\n")

    fh.write(f"\n## P1 Quality Issues: {total_p1}\n\n")
    for p in problems_p1:
        loc = f" (`{p['file']}`)" if p['file'] else ""
        fh.write(f"- **[P1]** {p['message']}{loc}\n")

    fh.write(f"\n## P2 Optimization Issues: {total_p2}\n\n")
    for p in problems_p2:
        loc = f" (`{p['file']}`)" if p['file'] else ""
        fh.write(f"- **[P2]** {p['message']}{loc}\n")

    fh.write(f"\n## Summary\n\n")
    fh.write(f"| Level | Count |\n")
    fh.write(f"|-------|-------|\n")
    fh.write(f"| P0    | {total_p0} |\n")
    fh.write(f"| P1    | {total_p1} |\n")
    fh.write(f"| P2    | {total_p2} |\n")

    fh.write(f"\n## Conclusion\n\n")
    if total_p0 > 0:
        fh.write(f"**FAIL**: {total_p0} blocking issues found.\n")
    else:
        fh.write(f"**PASS**: No blocking issues found.\n")

print(f"Structure audit report written to {report_path}")
print(f"P0={total_p0}, P1={total_p1}, P2={total_p2}")

sys.exit(1 if total_p0 > 0 else 0)
