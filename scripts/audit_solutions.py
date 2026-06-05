#!/usr/bin/env python3
"""Audit solutions coverage: check EXERCISES.md vs SOLUTIONS.md completeness."""

import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPORT_DIR = os.path.join(ROOT, "audit_reports")
os.makedirs(REPORT_DIR, exist_ok=True)

CHAPTERS = [f"chapters/{d}" for d in sorted(os.listdir(os.path.join(ROOT, "chapters"))) if os.path.isdir(os.path.join(ROOT, "chapters", d))]
PROJECTS = [f"projects/{d}" for d in sorted(os.listdir(os.path.join(ROOT, "projects"))) if os.path.isdir(os.path.join(ROOT, "projects", d))]
ALL_DIRS = CHAPTERS + PROJECTS

CORE_CHAPTERS = {
    "04_stack_heap_and_raii", "05_ownership_move_copy_clone",
    "06_references_borrowing_slices", "09_enums_option_pattern_matching",
    "12_error_handling_result_question_mark", "15_generics_traits_trait_bounds",
    "16_lifetimes", "19_smart_pointers_box_rc_refcell",
    "21_threads_channels_shared_state", "22_async_await_tokio_intro",
    "24_unsafe_rust_and_ffi_overview",
}

FORBIDDEN_PHRASES = [
    "Rust 完全没有运行时开销", "Rust 完全不使用堆", "Rust 所有变量都在栈上",
    "Move 就是深拷贝", "Rust Move 就是 C++ std::move", "遇到所有权问题直接 clone",
    "Rust 引用就是 C 指针", "生命周期标注会让变量活得更久", "'static 就是永远不会释放",
    "Option 只是换名字的 null", "Result 就是异常", "Trait 就是接口",
    "Trait 就是抽象类", "Arc 自动保证内部数据线程安全", "RefCell 可以绕过 Rust 安全规则",
    "Rust 不会发生内存泄漏", "Rust 不可能死锁", "Rust 编译通过就绝对没有 Bug",
    "异步就是多线程", "Tokio 是 Rust 标准库", "unsafe 会关闭全部检查",
    "使用 unsafe 一定是错误设计", "Cargo Feature 是运行时配置开关",
    "C++ 没有 RAII", "Rust 一定比 C++ 更快", "Python 一定比 Rust 更慢",
]

PLACEHOLDER_PATTERNS = ["TODO", "TBD", "待补充", "略", "自行完成", "后续补充"]

results = []

for dir_path in ALL_DIRS:
    full_dir = os.path.join(ROOT, dir_path)
    dir_name = os.path.basename(dir_path)
    has_exercises = os.path.exists(os.path.join(full_dir, "EXERCISES.md"))
    has_solutions = os.path.exists(os.path.join(full_dir, "SOLUTIONS.md"))

    entry = {
        "dir": dir_path, "name": dir_name,
        "has_exercises": has_exercises, "has_solutions": has_solutions,
        "is_core": dir_name in CORE_CHAPTERS,
        "exercise_count": 0, "solution_size": 0,
        "missing_answers": 0, "placeholders": [],
        "forbidden_hits": [],
        "is_project": dir_path.startswith("projects/"),
    }

    if has_exercises:
        with open(os.path.join(full_dir, "EXERCISES.md"), "r", encoding="utf-8") as f:
            ex_content = f.read()
        # Count exercise headings (## or ###)
        entry["exercise_count"] = len(re.findall(r'^#{2,3}\s+(练习|Exercise|Level|迁移|思考|编程)', ex_content, re.MULTILINE))

    if has_solutions:
        with open(os.path.join(full_dir, "SOLUTIONS.md"), "r", encoding="utf-8") as f:
            sol_content = f.read()
        entry["solution_size"] = len(sol_content)

        for phrase in FORBIDDEN_PHRASES:
            if phrase in sol_content:
                # Check if it's in a negation context (e.g., "不是 X 就是 Y")
                context_re = re.compile(r'(?:不是|不等于|并非|误解|错误).{0,20}' + re.escape(phrase))
                if not context_re.search(sol_content):
                    entry["forbidden_hits"].append(phrase)

        for pat in PLACEHOLDER_PATTERNS:
            for lineno, line in enumerate(sol_content.split("\n"), 1):
                if pat in line and pat != "略":
                    entry["placeholders"].append(f"L{lineno}: {line.strip()[:80]}")

        # Count answer sections
        answer_count = len(re.findall(r'^#{2,4}\s*(?:练习|答案|思路|参考实现|Level)', sol_content, re.MULTILINE))
        if entry["exercise_count"] > 0 and answer_count < entry["exercise_count"] * 0.5:
            entry["missing_answers"] = entry["exercise_count"] - answer_count

    results.append(entry)

# Core chapter specific checks
core_checks = {}
for r in results:
    if not r["is_core"] or not r["has_solutions"]:
        continue
    full_dir = os.path.join(ROOT, r["dir"])
    with open(os.path.join(full_dir, "SOLUTIONS.md"), "r", encoding="utf-8") as f:
        sol = f.read()

    checks = {}
    if r["name"] == "16_lifetimes":
        checks["生命周期标注不延长寿命"] = "生命周期标注不" in sol or "不改变" in sol
    if r["name"] == "05_ownership_move_copy_clone":
        checks["Move不是深拷贝"] = "不是深拷贝" in sol or "Move 不是" in sol
    if r["name"] in ("19_smart_pointers_box_rc_refcell", "21_threads_channels_shared_state"):
        checks["Arc不自动保证线程安全"] = "不自动保证" in sol or "不等于自动" in sol
    if r["name"] == "22_async_await_tokio_intro":
        checks["Async不等于多线程"] = "不等于多线程" in sol or "不等于线程" in sol or "不是多线程" in sol or "不是线程" in sol
    if r["name"] == "24_unsafe_rust_and_ffi_overview":
        checks["unsafe不关闭所有检查"] = "不关闭" in sol and "检查" in sol
    if checks:
        core_checks[r["name"]] = checks

# Generate report
report_path = os.path.join(REPORT_DIR, "solutions_report.md")
with open(report_path, "w", encoding="utf-8") as fh:
    fh.write("# Solutions Audit Report\n\n")
    fh.write(f"**Chapters**: {len(CHAPTERS)}\n")
    fh.write(f"**Projects**: {len(PROJECTS)}\n")
    fh.write(f"**Total packages**: {len(ALL_DIRS)}\n\n")

    fh.write(f"## Coverage Summary\n\n")
    fh.write("| Dir | Exercises | Solutions | Size | Missing | Placeholders | Forbidden |\n")
    fh.write("|-----|-----------|-----------|------|---------|-------------|----------|\n")
    total_ex = 0
    total_sol = 0
    total_missing = 0
    for r in results:
        ex = "✅" if r["has_exercises"] else "❌"
        sol = "✅" if r["has_solutions"] else "❌"
        fh.write(f"| {r['dir']} | {ex} ({r['exercise_count']}) | {sol} | {r['solution_size']}B | {r['missing_answers']} | {len(r['placeholders'])} | {len(r['forbidden_hits'])} |\n")
        total_ex += r["exercise_count"]
        if r["has_solutions"]:
            total_sol += 1
        total_missing += r["missing_answers"]

    fh.write(f"\n**Totals**: {total_ex} exercises, {total_sol}/{len(ALL_DIRS)} SOLUTIONS.md, ~{total_missing} missing answers\n\n")

    # Missing solutions
    missing = [r for r in results if not r["has_solutions"]]
    if missing:
        fh.write(f"## Missing SOLUTIONS.md ({len(missing)})\n\n")
        for r in missing:
            fh.write(f"- {r['dir']}\n")
        fh.write("\n")

    # Placeholders
    placeholder_hits = [r for r in results if r["placeholders"]]
    if placeholder_hits:
        fh.write(f"## Placeholders Found ({sum(len(r['placeholders']) for r in placeholder_hits)})\n\n")
        for r in placeholder_hits:
            fh.write(f"### {r['dir']}\n")
            for p in r["placeholders"]:
                fh.write(f"- {p}\n")
            fh.write("\n")

    # Forbidden phrases
    forbidden_hits = [r for r in results if r["forbidden_hits"]]
    if forbidden_hits:
        fh.write(f"## Forbidden Phrases ({sum(len(r['forbidden_hits']) for r in forbidden_hits)})\n\n")
        for r in forbidden_hits:
            fh.write(f"### {r['dir']}\n")
            for f in r["forbidden_hits"]:
                fh.write(f"- `{f}`\n")
            fh.write("\n")

    # Core chapter checks
    if core_checks:
        fh.write("## Core Chapter Accuracy Checks\n\n")
        for ch, checks in core_checks.items():
            fh.write(f"### {ch}\n")
            for label, ok in checks.items():
                fh.write(f"- {label}: {'✅' if ok else '❌ MISSING'}\n")
            fh.write("\n")

    # Final verdict
    blocker = any(r["forbidden_hits"] for r in results)
    serious_missing = len(missing) > 5
    if blocker or serious_missing:
        fh.write("**FAIL**: Significant issues found.\n")
    else:
        fh.write("**PASS**: No blocking issues.\n")

print(f"Solutions audit: {report_path}")
print(f"Total exercises: {total_ex}")
print(f"SOLUTIONS.md present: {total_sol}/{len(ALL_DIRS)}")
print(f"Missing: {len(missing)}")
print(f"Placeholders: {sum(len(r['placeholders']) for r in results)}")
print(f"Forbidden: {sum(len(r['forbidden_hits']) for r in results)}")

sys.exit(1 if blocker else 0)
