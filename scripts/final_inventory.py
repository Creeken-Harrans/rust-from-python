#!/usr/bin/env python3
"""Final inventory scan for rust-from-python tutorial.
Scans chapters/ and projects/ directories and outputs inventory.md.
Uses only Python 3 standard library.
"""

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CHAPTERS = ROOT / "chapters"
PROJECTS = ROOT / "projects"
REPORT_DIR = ROOT / "final_audit_reports"
REPORT_DIR.mkdir(exist_ok=True)

REQUIRED_CHAPTERS = [
    "00_course_orientation",
    "01_hello_cargo",
    "02_variables_and_types",
    "03_functions_expressions_control_flow",
    "04_stack_heap_and_raii",
    "05_ownership_move_copy_clone",
    "06_references_borrowing_slices",
    "07_ownership_practice_text_analyzer",
    "08_structs_methods_associated_functions",
    "09_enums_option_pattern_matching",
    "10_collections_vec_string_hashmap",
    "11_patterns_and_destructuring",
    "12_error_handling_result_question_mark",
    "13_packages_crates_modules_visibility",
    "14_testing_documentation_benchmindset",
    "15_generics_traits_trait_bounds",
    "16_lifetimes",
    "17_trait_objects_dynamic_dispatch",
    "18_closures_iterators",
    "19_smart_pointers_box_rc_refcell",
    "20_resource_management_drop_deref",
    "21_threads_channels_shared_state",
    "22_async_await_tokio_intro",
    "23_macros",
    "24_unsafe_rust_and_ffi_overview",
    "25_cargo_dependencies_features_profiles",
    "26_workspace_architecture",
    "27_lints_format_docs_ci",
]

REQUIRED_PROJECTS = [
    "01_guessing_game",
    "02_cli_text_search",
    "03_todo_cli",
    "04_parallel_text_stats",
    "05_mini_kv_store",
]

REQUIRED_PER_DIR = [
    "Cargo.toml",
    "README.md",
    "EXERCISES.md",
    "SOLUTIONS.md",
]

def check_dir(d: Path) -> dict:
    result = {
        "path": str(d.relative_to(ROOT)),
        "exists": d.is_dir(),
        "files": {},
        "missing": [],
        "has_src": False,
        "has_main_or_lib": False,
        "examples": [],
        "reference_solution": False,
    }
    if not d.is_dir():
        result["missing"].append("directory")
        return result

    for fname in REQUIRED_PER_DIR:
        fp = d / fname
        if fp.is_file():
            size = fp.stat().st_size
            result["files"][fname] = {"size": size, "empty": size == 0}
        else:
            result["missing"].append(fname)

    src_dir = d / "src"
    result["has_src"] = src_dir.is_dir()
    if src_dir.is_dir():
        has_main = (src_dir / "main.rs").is_file()
        has_lib = (src_dir / "lib.rs").is_file()
        result["has_main_or_lib"] = has_main or has_lib
        if not has_main and not has_lib:
            result["missing"].append("src/main.rs or src/lib.rs")

    examples_dir = d / "examples"
    if examples_dir.is_dir():
        for f in examples_dir.glob("*.rs"):
            result["examples"].append(str(f.relative_to(d)))

    ref_dir = d / "reference_solution"
    result["reference_solution"] = ref_dir.is_dir()

    return result

def count_all_markdown() -> int:
    return len(list(ROOT.glob("**/*.md")))

def count_rust_files() -> int:
    rs_files = list(ROOT.glob("chapters/**/*.rs")) + \
                list(ROOT.glob("projects/**/*.rs")) + \
                list(ROOT.glob("broken_examples/**/*.rs"))
    return len(rs_files)

def has_placeholder(content: str):
    """Check if file content has placeholder/template patterns."""
    markers = ["TODO", "TBD", "PLACEHOLDER", "待补充", "后续补充", "稍后完成",
               "自行完成", "自行实现", "暂缺", "未实现", "以后再写"]
    lines = content.split("\n")
    for i, line in enumerate(lines, 1):
        for m in markers:
            if m in line:
                return True, i, m
    return False, 0, ""

def main():
    output = []
    output.append("# Final Release Audit — Directory Inventory\n")
    output.append(f"**Date**: 2026-06-05\n")
    output.append(f"**Root**: {ROOT}\n")
    output.append("")

    # --- Chapters ---
    output.append("## Chapters\n")
    chapter_results = {}
    missing_chapters = []
    for name in REQUIRED_CHAPTERS:
        d = CHAPTERS / name
        r = check_dir(d)
        chapter_results[name] = r
        if not r["exists"]:
            missing_chapters.append(name)

    # --- Projects ---
    output.append("## Projects\n")
    project_results = {}
    missing_projects = []
    for name in REQUIRED_PROJECTS:
        d = PROJECTS / name
        r = check_dir(d)
        project_results[name] = r
        if not r["exists"]:
            missing_projects.append(name)

    # --- Statistics ---
    output.append("## Statistics\n")

    total_chapters = len(REQUIRED_CHAPTERS)
    total_projects = len(REQUIRED_PROJECTS)
    cargo_packages = len([r for r in chapter_results.values() if r["exists"] and "Cargo.toml" in r["files"]]) + \
                     len([r for r in project_results.values() if r["exists"] and "Cargo.toml" in r["files"]])

    readme_count = sum(1 for r in chapter_results.values() if "README.md" in r["files"]) + \
                   sum(1 for r in project_results.values() if "README.md" in r["files"])
    exercises_count = sum(1 for r in chapter_results.values() if "EXERCISES.md" in r["files"]) + \
                      sum(1 for r in project_results.values() if "EXERCISES.md" in r["files"])
    solutions_count = sum(1 for r in chapter_results.values() if "SOLUTIONS.md" in r["files"]) + \
                      sum(1 for r in project_results.values() if "SOLUTIONS.md" in r["files"])

    example_count = sum(len(r["examples"]) for r in chapter_results.values()) + \
                    sum(len(r["examples"]) for r in project_results.values())
    ref_sol_count = sum(1 for r in chapter_results.values() if r["reference_solution"]) + \
                    sum(1 for r in project_results.values() if r["reference_solution"])

    md_count = count_all_markdown()
    rs_count = count_rust_files()

    # Count missing
    all_missing = []
    for name, r in chapter_results.items():
        for m in r["missing"]:
            all_missing.append(f"chapters/{name}/{m}")
    for name, r in project_results.items():
        for m in r["missing"]:
            all_missing.append(f"projects/{name}/{m}")

    # Count empty files
    empty_files = []
    for name, r in chapter_results.items():
        for fname, finfo in r["files"].items():
            if finfo["empty"]:
                empty_files.append(f"chapters/{name}/{fname}")
    for name, r in project_results.items():
        for fname, finfo in r["files"].items():
            if finfo["empty"]:
                empty_files.append(f"projects/{name}/{fname}")

    # Check for placeholders in SOLUTIONS.md
    placeholder_hits = []
    for name, r in chapter_results.items():
        sol_path = CHAPTERS / name / "SOLUTIONS.md"
        if sol_path.is_file():
            content = sol_path.read_text(encoding="utf-8")
            hit, line, marker = has_placeholder(content)
            if hit:
                placeholder_hits.append(f"chapters/{name}/SOLUTIONS.md:{line}: {marker}")
    for name, r in project_results.items():
        sol_path = PROJECTS / name / "SOLUTIONS.md"
        if sol_path.is_file():
            content = sol_path.read_text(encoding="utf-8")
            hit, line, marker = has_placeholder(content)
            if hit:
                placeholder_hits.append(f"projects/{name}/SOLUTIONS.md:{line}: {marker}")

    output.append("| Metric | Count |")
    output.append("|--------|------:|")
    output.append(f"| Chapter directories | {total_chapters} |")
    output.append(f"| Project directories | {total_projects} |")
    output.append(f"| Cargo Packages | {cargo_packages} |")
    output.append(f"| README.md | {readme_count} |")
    output.append(f"| EXERCISES.md | {exercises_count} |")
    output.append(f"| SOLUTIONS.md | {solutions_count} |")
    output.append(f"| examples/*.rs | {example_count} |")
    output.append(f"| reference_solution/ | {ref_sol_count} |")
    output.append(f"| Markdown files (total) | {md_count} |")
    output.append(f"| Rust source files | {rs_count} |")
    output.append(f"| Missing files | {len(all_missing)} |")
    output.append(f"| Empty files | {len(empty_files)} |")
    output.append(f"| Placeholder hits | {len(placeholder_hits)} |")
    output.append("")

    if missing_chapters:
        output.append("### Missing Chapters\n")
        for name in missing_chapters:
            output.append(f"- [ ] chapters/{name}")
        output.append("")

    if missing_projects:
        output.append("### Missing Projects\n")
        for name in missing_projects:
            output.append(f"- [ ] projects/{name}")
        output.append("")

    if all_missing:
        output.append("### Missing Files\n")
        for f in sorted(all_missing):
            output.append(f"- [ ] `{f}`")
        output.append("")

    if empty_files:
        output.append("### Empty Files\n")
        for f in sorted(empty_files):
            output.append(f"- [ ] `{f}`")
        output.append("")

    if placeholder_hits:
        output.append("### Placeholder Hits in SOLUTIONS.md\n")
        for h in sorted(placeholder_hits):
            output.append(f"- [ ] `{h}`")
        output.append("")

    # Per-directory detail
    output.append("## Per-Chapter Detail\n")
    output.append("| Chapter | Cargo.toml | README.md | EXERCISES.md | SOLUTIONS.md | src/ | Missing |")
    output.append("|---------|:----------:|:---------:|:------------:|:------------:|:----:|:-------:|")
    for name in REQUIRED_CHAPTERS:
        r = chapter_results[name]
        if not r["exists"]:
            output.append(f"| {name} | ❌ DIR MISSING | | | | | |")
            continue
        ct = "✅" if "Cargo.toml" in r["files"] else "❌"
        rm = "✅" if "README.md" in r["files"] else "❌"
        ex = "✅" if "EXERCISES.md" in r["files"] else "❌"
        sl = "✅" if "SOLUTIONS.md" in r["files"] else "❌"
        sr = "✅" if r["has_main_or_lib"] else "❌"
        ms = ", ".join(r["missing"]) if r["missing"] else "—"
        output.append(f"| {name} | {ct} | {rm} | {ex} | {sl} | {sr} | {ms} |")

    output.append("")
    output.append("## Per-Project Detail\n")
    output.append("| Project | Cargo.toml | README.md | EXERCISES.md | SOLUTIONS.md | src/ | Missing |")
    output.append("|---------|:----------:|:---------:|:------------:|:------------:|:----:|:-------:|")
    for name in REQUIRED_PROJECTS:
        r = project_results[name]
        if not r["exists"]:
            output.append(f"| {name} | ❌ DIR MISSING | | | | | |")
            continue
        ct = "✅" if "Cargo.toml" in r["files"] else "❌"
        rm = "✅" if "README.md" in r["files"] else "❌"
        ex = "✅" if "EXERCISES.md" in r["files"] else "❌"
        sl = "✅" if "SOLUTIONS.md" in r["files"] else "❌"
        sr = "✅" if r["has_main_or_lib"] else "❌"
        ms = ", ".join(r["missing"]) if r["missing"] else "—"
        output.append(f"| {name} | {ct} | {rm} | {ex} | {sl} | {sr} | {ms} |")

    report = "\n".join(output)

    out_path = REPORT_DIR / "inventory.md"
    out_path.write_text(report, encoding="utf-8")
    print(f"Inventory written to {out_path}")
    print(f"  Chapters: {total_chapters} (missing: {len(missing_chapters)})")
    print(f"  Projects: {total_projects} (missing: {len(missing_projects)})")
    print(f"  Cargo Packages: {cargo_packages}")
    print(f"  README.md: {readme_count}/33")
    print(f"  EXERCISES.md: {exercises_count}/33")
    print(f"  SOLUTIONS.md: {solutions_count}/33")
    print(f"  Missing files: {len(all_missing)}")
    print(f"  Empty files: {len(empty_files)}")
    print(f"  Placeholder hits: {len(placeholder_hits)}")

    # Return non-zero if issues
    exit_code = 0
    if missing_chapters or missing_projects:
        exit_code = 1
        print("ERROR: Missing chapter or project directories!")
    if all_missing:
        exit_code = 1
        print("ERROR: Missing required files!")
    if placeholder_hits:
        exit_code = 1
        print("ERROR: SOLUTIONS.md contains placeholder text!")

    sys.exit(exit_code)

if __name__ == "__main__":
    main()
